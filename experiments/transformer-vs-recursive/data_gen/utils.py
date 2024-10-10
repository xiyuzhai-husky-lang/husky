import numpy as np
import tiktoken
from tqdm import tqdm

FUNC_NAMES = [
    "apple", "banana", "cat", "dog", "elephant", "frog", "giraffe", "hat", "ice", "jug",
    "kite", "lemon", "mango", "notebook", "orange", "pencil", "quilt", "rose", "sunflower",
    "table", "umbrella", "vase", "watermelon", "yacht", "zebra", "asparagus",
    "bottle", "cucumber", "desk", "earrings", "flute", "grapes", "helicopter",
    "jacket", "kiwi", "lamp", "mirror", "nest", "octopus", "pillow", "rabbit",
    "shoe", "turtle", "unicorn", "violin", "walnut", "yogurt", "zipper", "acorn",
    "backpack", "candle", "dolphin", "eggplant", "fence", "glove", "hamburger", "island",
    "jeans", "kangaroo", "lighthouse", "mushroom", "note", "owl", "pumpkin", "quill",
    "rocket", "strawberry", "teapot", "urchin", "violet", "whale", "yam", 
    "anchor", "brush", "clock", "daisy", "envelope", "fox", "guitar", "house", "iceberg",
    "jar", "koala", "leaf", "mountain", "nail", "ocean", "piano", "queen", "ring", "star",
    "tree", "ukulele", "volcano", "windmill", "zucchini", "armchair",
    "book", "couch", "door", "escalator", "fan", "gate", "helmet", "ink", "jellyfish",
    "lion", "mouse", "nectarine", "ostrich", "peach", "quartz", "rosemary", "sandal",
    "television", "urchin", "vaccine", "window", "beetle",
    "coral", "dandelion", "eagle", "fern", "harp", "jade", "kettle", "llama",
    "maple", "nutmeg", "orchid", "pepper", "quiver", "radish", "seagull", "tulip", "urchin",
    "van", "wasp", "avocado", "carrot", "duck", "eel",
    "fig", "goose", "leopard", "mole",
    "pea", "quinoa", "raccoon", "squirrel", "toad", "urchin", "weasel",
]
VAR_NAMES = [
    'i', 'j', 'k', 'x', 'y', 'z', 'n', 'm', 't', 'val', 'value',
    'num', 'number', 'data', 'result', 'results', 'item', 'items',
    'elem', 'element', 'elements', 'list', 'lst', 'array', 'arr',
    'string', 'str', 'char', 'word', 'text', 'line', 'lines',
    'name', 'names', 'firstname', 'lastname', 'email', 'username',
    'password', 'url', 'path', 'dir', 'file', 'filename', 'filepath',
    'count', 'total', 'sum', 'avg', 'average', 'min', 'max',
    'length', 'len', 'size', 'index', 'pos', 'position', 'start',
    'end', 'status', 'state', 'flag', 'error', 'msg', 'message',
    'title', 'body', 'content', 'description', 'type', 'kind',
    'id', 'key', 'keys', 'value', 'values', 'data', 'payload',
    'json', 'xml', 'html', 'tag', 'tags', 'category', 'categories',
    'group', 'groups', 'batch', 'record', 'records', 'field', 'fields',
    'object', 'obj', 'instance', 'model', 'dict', 'dictionary', 'map',
    'mapping', 'table', 'queue', 'stack', 'tree', 'node', 'leaf',
    'branch', 'root', 'edge', 'vertex', 'vertices', 'graph',
    'function', 'func', 'method', 'algorithm', 'procedure',
    'action', 'activity', 'task', 'job', 'process', 'thread',
    'command', 'request', 'response', 'reply', 'output', 'input',
    'config', 'configuration', 'setting', 'settings', 'option',
    'options', 'param', 'parameter', 'parameters', 'arg', 'args',
    'argument', 'arguments', 'env', 'environment', 'version',
    'date', 'time', 'timestamp', 'year', 'month', 'day', 'hour',
    'minute', 'second', 'millisecond', 'timezone', 'period', 'duration',
    'interval', 'frequency', 'rate', 'ratio', 'percentage', 'percent',
    'score', 'point', 'points', 'grade', 'rating', 'rank',
    'level', 'stage', 'phase', 'cycle', 'sequence', 'list',
    'series', 'array', 'collection', 'bundle', 'packet', 'set',
    'batch', 'bundle', 'heap', 'pile', 'stack', 'queue', 'deque',
    'link', 'connection', 'channel', 'socket', 'port', 'protocol',
    'format', 'pattern', 'template', 'schema', 'outline', 'framework',
    'structure', 'system', 'platform', 'engine', 'tool', 'utility',
    'application', 'app', 'software', 'package', 'library', 'module',
    'script', 'code', 'program', 'project', 'repository', 'repo',
    'branch', 'version', 'release', 'patch', 'update', 'upgrade',
    'fix', 'bug', 'issue', 'error', 'warning', 'notice', 'alert',
    'log', 'report', 'history', 'record', 'comment', 'note', 'annotation'
]

INT_LITERALS = [str(i) for i in range(99)]
FLOAT_LITERALS = [f"{i}.1" for i in range(99)]
BOOL_LITERALS = ["true", "false"]

tokenizer = tiktoken.encoding_for_model("gpt2")

class AstKind:
    FnEntityName, ParameterType, ParameterIdent, FnEntityUsage, CallLpar, FnKeyword, LetKeyword, CallRpar, IntLiteral, FloatLiteral, BoolLiteral, ParametersLpar, ParametersRpar, FnBodyLcurl, FnBodyRcurl, ParameterTypeColon, StmtColon = range(1, 18)

class SymbolResolution:
    Fn, Unresolved = 1, 2

class TypeError:
    Expected = 1

class Type:
    Bool, Int, Float = 1, 2, 3

    @classmethod
    def repr(cls, value):
        if value == cls.Bool:
            return "Bool"
        elif value == cls.Int:
            return "Int"
        elif value == cls.Float:
            return "Float"

class Function:
    def __init__(self, name, input_ty, last_called_step):
        self.name = name
        self.input_ty = input_ty
        self.last_called_step = last_called_step

class BasicCodeGenerator:
    def __init__(self, seed, min_dist, use_var_rate, error_rate):
        self.rng = np.random.default_rng(seed)
        self.min_dist = min_dist
        self.use_var_rate = use_var_rate
        self.error_rate = error_rate
        self.max_calls_per_fn = 10
        self.used_fn_idx = []
        self.functions = []
        self.result = {
            "text": "",
            "token": [],
            "ast_kind": [],
            "symbol_resolution": [],
            "expected_type": [],
            "actual_type": []
        }
        self.time_step = 0

    def push_token(self, token, ast_kind=0, symbol_resolution=0, expected_type=0, actual_type=0):
        if self.result["text"]:
            self.result["text"] += " "
        self.result["text"] += token
        tokens = tokenizer.encode(token)
        for t in tokens:
            self.result["token"].append(t)
            self.result["ast_kind"].append(ast_kind)
            self.result["symbol_resolution"].append(symbol_resolution)
            self.result["expected_type"].append(expected_type)
            self.result["actual_type"].append(actual_type)

    def with_curly(self, lcurl_kind, rcurl_kind, f):
        self.push_token("{", lcurl_kind)
        f(self)
        self.push_token("}", rcurl_kind)

    def gen_fn(self):
        fn_idx = self.rng.integers(low=0, high=len(FUNC_NAMES))
        while fn_idx in self.used_fn_idx:
            fn_idx = self.rng.integers(low=0, high=len(FUNC_NAMES))
        self.used_fn_idx.append(fn_idx)
        fn_name = FUNC_NAMES[fn_idx]
        input_ty = self.rng.choice([Type.Bool, Type.Int, Type.Float]).item()
        var_name = self.rng.choice(VAR_NAMES).item()

        self.push_token("fn", AstKind.FnKeyword)
        self.push_token(fn_name, AstKind.FnEntityName, SymbolResolution.Fn)
        self.push_token("(", AstKind.ParametersLpar)
        self.push_token(var_name, AstKind.ParameterIdent, expected_type=input_ty, actual_type=input_ty)
        self.push_token(":", AstKind.ParameterTypeColon)
        self.push_token(Type.repr(input_ty), AstKind.ParameterType, expected_type=input_ty, actual_type=input_ty)
        self.push_token(")", AstKind.ParametersRpar)
        self.with_curly(AstKind.FnBodyLcurl, AstKind.FnBodyRcurl, lambda gen: self.gen_fn_body(var_name, input_ty))
        self.functions.append(Function(fn_name, input_ty, self.time_step))
        self.time_step += 1

    def gen_fn_body(self, var_name, input_ty):
        for _ in range(self.max_calls_per_fn):
            self.gen_fn_call(var_name, input_ty)

    def gen_fn_call(self, var_name, var_type):
        if not self.functions:
            return
        callee_index = self.rng.integers(low=0, high=len(self.functions))
        callee = self.functions[callee_index]

        if self.time_step - callee.last_called_step < self.min_dist:
            return
        callee.last_called_step = self.time_step

        fn_name = callee.name
        expected_type = callee.input_ty
        value_type = expected_type

        use_var = self.rng.random() < self.use_var_rate
        if use_var:
            arg_literal = var_name
            literal_kind = AstKind.ParameterIdent
            value_type = var_type
        else:
            has_ty_error = self.rng.random() < self.error_rate
            if has_ty_error:
                types = [Type.Bool, Type.Int, Type.Float]
                types.remove(expected_type)
                value_type = self.rng.choice(types).item()
            arg_literal, literal_kind = self.get_literal_by_type(value_type)

        self.push_token(fn_name, AstKind.FnEntityUsage, SymbolResolution.Fn)
        self.push_token("(", AstKind.CallLpar)
        self.push_token(arg_literal, literal_kind, expected_type=expected_type, actual_type=value_type)
        self.push_token(")", AstKind.CallRpar)
        self.push_token(";", AstKind.StmtColon)

    def get_literal_by_type(self, value_type):
        if value_type == Type.Bool:
            return self.rng.choice(BOOL_LITERALS).item(), AstKind.BoolLiteral
        elif value_type == Type.Int:
            return self.rng.choice(INT_LITERALS).item(), AstKind.IntLiteral
        elif value_type == Type.Float:
            return self.rng.choice(FLOAT_LITERALS).item(), AstKind.FloatLiteral

    def gen_fns(self, max_fns):
        num_fns = self.rng.integers(low=3, high=max(max_fns, 3) + 1)
        for _ in range(num_fns):
            self.gen_fn()

    def finish(self):
        return self.result

from concurrent.futures import ProcessPoolExecutor
from tqdm import tqdm

def generate_code(seed, max_fns, min_dist, use_var_rate, error_rate):
    """
    This function is a wrapper for the code generation process,
    designed to be called by each worker in the pool.
    """
    bcg = BasicCodeGenerator(seed, min_dist, use_var_rate, error_rate)
    bcg.gen_fns(max_fns)
    return bcg.finish()

def rnd_codes_parallel(n, max_fns, min_dist, use_var_rate, error_rate, workers=None):
    """
    Generate random codes in parallel.

    :param n: Number of codes to generate.
    :param max_fns: Maximum number of functions.
    :param min_dist: Minimum distance.
    :param use_var_rate: Use variable rate flag.
    :param error_rate: Error rate.
    :param workers: Number of worker processes to use. If None, it will use the number of processors on the machine.
    :return: List of generated codes.
    """
    # Tuple of arguments to pass to each parallel invocation
    args = [(seed, max_fns, min_dist, use_var_rate, error_rate) for seed in range(n)]
    
    # Use ProcessPoolExecutor to execute calls asynchronously
    with ProcessPoolExecutor(max_workers=workers) as executor:
        # Map the generate_code function to the arguments asynchronously
        # tqdm can be integrated with map() to show progress
        results = list(tqdm(executor.map(generate_code, *zip(*args), chunksize=1024), total=n))
    
    return results
