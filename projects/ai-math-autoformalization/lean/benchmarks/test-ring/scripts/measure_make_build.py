#! /usr/bin/env python3
import subprocess
import time

def imports():
    return """
import Mathlib.Algebra.Ring.Basic
import Mathlib.Tactic.Ring
"""

def gen_namespace(n: int):
    return f"""
namespace MyTheormes{n}

-- Basic properties of addition and multiplication
theorem theorem1 (a b c : ℤ) : a + b + c = a + (b + c) :=
by ring

theorem theorem2 (a b c : ℤ) : a * b * c = a * (b * c) :=
by ring

theorem theorem3 (a b c : ℤ) : a * (b + c) = a * b + a * c :=
by ring

theorem theorem4 (a b c : ℤ) : (a + b) * c = a * c + b * c :=
by ring

theorem theorem5 (a b : ℤ) : a + b = b + a :=
by ring

theorem theorem6 (a b : ℤ) : a * b = b * a :=
by ring

-- Properties of zero
theorem theorem7 (a : ℤ) : a + 0 = a :=
by ring

theorem theorem8 (a : ℤ) : 0 + a = a :=
by ring

theorem theorem9 (a : ℤ) : a * 0 = 0 :=
by ring

theorem theorem10 (a : ℤ) : 0 * a = 0 :=
by ring

-- Properties of negation
theorem theorem11 (a : ℤ) : -a + a = 0 :=
by ring

theorem theorem12 (a : ℤ) : a + -a = 0 :=
by ring

-- Distributivity
theorem theorem13 (a b c : ℤ) : a * (b + c) = a * b + a * c :=
by ring

theorem theorem14 (a b c : ℤ) : (a + b) * c = a * c + b * c :=
by ring

-- Other simple identities
theorem theorem15 (a b : ℤ) : (a + b) * (a + b) = a * a + 2 * a * b + b * b :=
by ring

theorem theorem16 (a b : ℤ) : (a + b) * (a + b) * (a + b) = a^3 + 3 * a^2 * b + 3 * a * b^2 + b^3 :=
by ring

theorem theorem17 (a b : ℤ) : (a + b) * (a - b) = a^2 - b^2 :=
by ring

end MyTheormes{n}
"""

def gen_code(n: int):
    # Start with imports
    code = imports()
    
    # Add namespaces 1 through n, separated by blank lines
    for i in range(1, n + 1):
        code += gen_namespace(i) + "\n"
    
    return code

def prepare():
    # Then write empty file and build
    with open('TestRing/Basic.lean', 'w') as f:
        f.write('')
    
    subprocess.run(['make', 'build'], check=True)  # Just build, don't measure
    
    # Then write actual code
    with open('TestRing/Basic.lean', 'w') as f:
        f.write(gen_code(100))

def measure_build():
    start_time = time.time()
    
    try:
        # Run make build and capture output
        result = subprocess.run(['make', 'build'], 
                              check=True,
                              capture_output=True,
                              text=True)
        
        end_time = time.time()
        build_time = end_time - start_time
        
        print(f"Build completed successfully in {build_time:.2f} seconds")
        return True
        
    except subprocess.CalledProcessError as e:
        print(f"Build failed with error:\n{e.stderr}")
        return False

if __name__ == '__main__':
    prepare()
    measure_build()
