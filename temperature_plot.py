from os.path import exists
import matplotlib.pyplot as plt
from pathlib import Path
import numpy as np

def temperature_iterative(t: int, n: int) -> float:
    temp = 100 * np.log(n)
    decay_factor = np.exp(-1 / np.log(n))
    for _ in range(t):
        temp *= decay_factor
    return temp

def generate_points(n: int) -> tuple[list[int], list[float]]:
    x_coords = []
    y_coords = []
    x = 0
    while x < 2000: 
        val = temperature_iterative(x, n)
        x_coords.append(x)
        y_coords.append(val)
        if val <= 0.01:
            break
        x += 1
    return x_coords, y_coords

def main():
    out_dir = Path("figures/")
    if not out_dir.exists():
        out_dir.mkdir()
    file_path = out_dir / "decay_comparison.svg"
    n_s = [28, 39, 194, 734, 1929, 4663, 7000, 8000, 10000]
    
    plt.figure(figsize=(10, 6))
    
    for n in n_s:
        x, y = generate_points(n)
        
        plt.plot(x, y, label=f"n={n}")

    plt.title("Temperature Decay for Multiple n Values")
    plt.xlabel("Time (t)")
    plt.ylabel("Temperature")
    plt.legend() 
    plt.grid(True, linestyle='--', alpha=0.6)
    
    plt.savefig(file_path)
    plt.show()

if __name__ == "__main__":
    main()
