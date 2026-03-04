import os
import subprocess
import re
import time

EXECUTABLE = "./target/release/a"
INPUT_DIR = "./src/ahc/ahc060/in"

PARAMS = {
    "PARAM_PENALTY": "0.99",
    "PARAM_REWARD_BASE": "1.5",
    "PARAM_REWARD_COEF": "0.1",
    "PARAM_PROB_FACTOR": "0.5",
    "PARAM_FALLBACK_PROB": "0.3",
}

def main():
    if not os.path.exists(INPUT_DIR):
        print(f"Input directory not found: {INPUT_DIR}")
        return

    input_files = [
        os.path.join(INPUT_DIR, f) 
        for f in os.listdir(INPUT_DIR) 
        if f.endswith('.txt')
    ]
    input_files.sort()

    if not input_files:
        print("No test cases found.")
        return

    print(f"Running {len(input_files)} test cases sequentially...")
    print(f"Params: {PARAMS}\n")
    
    total_score = 0
    start_time = time.time()
    
    for input_file in input_files:
        file_name = os.path.basename(input_file)
        try:
            with open(input_file, 'r') as f:
                env = os.environ.copy()
                env.update(PARAMS)
                
                result = subprocess.run(
                    [EXECUTABLE],
                    stdin=f,
                    capture_output=True,
                    text=True,
                    env=env,
                    timeout=3.0
                )
            
            match = re.search(r'Score:\s*(\d+)', result.stderr)
            if match:
                score = int(match.group(1))
                print(f"{file_name}: {score}")
                total_score += score
            else:
                print(f"{file_name}: Warning - No score found")
                
        except subprocess.TimeoutExpired:
            print(f"{file_name}: Timeout")
        except Exception as e:
            print(f"{file_name}: Error - {e}")

    elapsed = time.time() - start_time
    print(f"\nTotal Score: {total_score}")
    print(f"Elapsed Time: {elapsed:.2f} s")

if __name__ == "__main__":
    main()