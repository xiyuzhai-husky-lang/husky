import subprocess
from pathlib import Path


class LeanSandbox:
    MAIN_FUNC = '''
def main : IO Unit :=
  IO.println "Success!"
'''

    def __init__(self, project_dir: str = "mathproof"):
        self.project_dir = project_dir

    def setup_lean_project(self):
        """Initialize a new Lean project and fetch mathlib."""
        try:
            print("Initializing Lean project...")
            subprocess.run(["lake", "init", self.project_dir], check=False)
            
            print("Fetching mathlib...")
            subprocess.run(["lake", "update"], check=True)
            
        except subprocess.CalledProcessError as e:
            print(f"Error setting up project: {e}")
            raise



    def run_lean_file(self, file_path: str) -> subprocess.CompletedProcess:
        """Run a Lean file and return the execution result."""
        result = subprocess.run(
            ["lake", "env", "lean", "--run", file_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        return result


    def generate_lean_checking_file(self, lean_text: str, lean_file_path: str):
        """Generate a Lean file with the provided text and main function."""
        file_path = Path(lean_file_path)
        
        # Ensure directory exists
        file_path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(file_path, "w") as f:
            f.write(lean_text + "\n" + self.MAIN_FUNC)
        
        print(f"Generated Lean file: {lean_file_path}")