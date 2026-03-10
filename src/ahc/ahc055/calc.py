import subprocess
import glob
import os

# Configuration
SOLVER_PATH = "./target/release/a" # Path to your compiled Rust binary
INPUT_DIR = "./src/ahc/ahc055/in/*.txt"

def calculate_score(input_file):
    # Read Hi from input file
    with open(input_file, "r") as f:
        lines = f.readlines()
        n = int(lines[0].strip())
        h_values = list(map(int, lines[1].split()))
        sum_h = sum(h_values)

    # Run the solver and capture output
    try:
        # Provide the input file content to stdin
        with open(input_file, "r") as f:
            result = subprocess.run(
                [SOLVER_PATH],
                stdin=f,
                capture_output=True,
                text=True,
                check=True
            )
        
        # T is the number of lines in the output
        output_lines = result.stdout.strip().split("\n")
        t = len(output_lines) if output_lines[0] != "" else 0
        
        # Score formula from the problem statement
        score = sum_h - t + 1
        return score
    except Exception as e:
        print(f"Error processing {input_file}: {e}")
        return None

def main():
    input_files = sorted(glob.glob(INPUT_DIR))
    if not input_files:
        print("No input files found.")
        return

    total_score = 0
    count = 0

    print(f"{'File':<20} | {'Score':<10}")
    print("-" * 33)

    for file_path in input_files:
        score = calculate_score(file_path)
        if score is not None:
            file_name = os.path.basename(file_path)
            print(f"{file_name:<20} | {score:<10}")
            total_score += score
            count += 1

    if count > 0:
        average_score = total_score / count
        print("-" * 33)
        print(f"Total Cases: {count}")
        print(f"Average Score: {average_score:.2f}")

if __name__ == "__main__":
    main()