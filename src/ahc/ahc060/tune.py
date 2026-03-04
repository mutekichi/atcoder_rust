import subprocess
import os
import concurrent.futures
import optuna
import re

# Set the path to your compiled Rust executable and input directory
EXECUTABLE = "./target/release/a" # Update this path
INPUT_DIR = "./src/ahc/ahc060/in" # Update this path
NUM_TESTCASES = 100 # Number of test cases to evaluate per trial

def run_solver(input_file, env_params):
    try:
        with open(input_file, 'r') as f:
            # Run the executable with the specified environment variables
            result = subprocess.run(
                [EXECUTABLE],
                stdin=f,
                capture_output=True,
                text=True,
                env={**os.environ, **env_params},
                timeout=3.0 # Set slightly longer than the 2.0s limit
            )
        
        # Extract the score from stderr
        match = re.search(r'Score:\s*(\d+)', result.stderr)
        if match:
            return int(match.group(1))
        else:
            return 0
    except Exception as e:
        print(f"Error running {input_file}: {e}")
        return 0

def objective(trial):
    # Define hyperparameter search spaces
    env_params = {
        "PARAM_PENALTY": str(trial.suggest_float("penalty", 0.5, 0.99)),
        "PARAM_REWARD_BASE": str(trial.suggest_float("reward_base", 1.0, 1.5)),
        "PARAM_REWARD_COEF": str(trial.suggest_float("reward_coef", 0.0, 0.5)),
        "PARAM_PROB_FACTOR": str(trial.suggest_float("prob_factor", 0.1, 1.0)),
        "PARAM_FALLBACK_PROB": str(trial.suggest_float("fallback_prob", 0.1, 0.8)),
    }

    input_files = [
        os.path.join(INPUT_DIR, f"{i:04d}.txt") 
        for i in range(NUM_TESTCASES)
    ]

    total_score = 0
    
    # Run test cases in parallel for speed
    with concurrent.futures.ThreadPoolExecutor() as executor:
        futures = [
            executor.submit(run_solver, file, env_params)
            for file in input_files
        ]
        for future in concurrent.futures.as_completed(futures):
            total_score += future.result()

    # Optuna tries to maximize this value
    return total_score

if __name__ == "__main__":
    study = optuna.create_study(direction="maximize")
    print("Starting optimization...")
    study.optimize(objective, n_trials=100) # Run 100 different parameter combinations

    print("\nBest trial:")
    trial = study.best_trial
    print(f"  Value: {trial.value}")
    print("  Params: ")
    for key, value in trial.params.items():
        print(f"    {key}: {value}")