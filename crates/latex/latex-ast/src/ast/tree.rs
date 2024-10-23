#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TexTreeAstData {}

#[test]
fn tex_tree_token_works() {
    "Let X be a topological space with the number of points being $a*b*c+c+d+1$";
    "[S [NP [N Let] [NP [X]]] [VP [V be] [NP [Det a] [AP [A topological] [N space]] [PP [P with] [NP [Det the] [N number] [PP [P of] [NP [N points] [VP [V being] [Expr a*b*c+c+d+1]]]]]]]]]]
    ";
}
