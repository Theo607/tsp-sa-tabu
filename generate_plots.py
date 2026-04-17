import os
import matplotlib.pyplot as plt

def load_tsp(filepath):
    coords = {}
    try:
        with open(filepath, 'r') as f:
            lines = f.readlines()
            section = False
            for line in lines:
                line = line.strip()
                if line == "NODE_COORD_SECTION":
                    section = True
                    continue
                if line in ["EOF", "-1"] or not line:
                    if section: break
                    continue
                if section:
                    parts = line.split()
                    if len(parts) >= 3:
                        coords[int(parts[0])] = (float(parts[1]), float(parts[2]))
    except Exception as e:
        print(f"Błąd wczytywania {filepath}: {e}")
    return coords

def load_tour(filepath):
    tour = []
    try:
        with open(filepath, 'r') as f:
            section = False
            for line in f:
                line = line.strip()
                if line == "TOUR_SECTION":
                    section = True
                    continue
                if line in ["-1", "EOF"]: break
                if section and line.replace('-', '').isdigit():
                    tour.append(int(line))
    except Exception as e:
        print(f"Błąd wczytywania trasy {filepath}: {e}")
    return tour

def plot_tour(name, coords, tour, output_path):
    plt.figure(figsize=(10, 10))
    path_nodes = tour + [tour[0]]
    
    c1 = [coords[node][0] for node in path_nodes] # Lat
    c2 = [coords[node][1] for node in path_nodes] # Lon

    
    if "ca4663" in name:
        x, y = [-v for v in c2], c1
    elif "zi929" in name:
        x, y = c2, [-v for v in c1]

    elif any(k in name for k in ["mu1979", "dj38", "qa194"]):
        x, y = c2, c1

    elif "uy734" in name:
        x, y = [-v for v in c2], [-v for v in c1]

    elif "wi29" in name:
        x, y = [-v for v in c2], c1
    else:
        x, y = c2, c1

    plt.plot(x, y, color='#2c3e50', linewidth=0.8, alpha=0.7)
    plt.scatter(x, y, color='#e74c3c', s=2)
    
    plt.gca().set_aspect('equal', adjustable='box')
    plt.axis('off')
    
    plt.savefig(output_path, format='svg', bbox_inches='tight')
    plt.close()
    print(f"Zakończono sukcesem: {name}")

def main():
    tsp_dir = "assets"
    tour_dir = "output"
    fig_dir = "figures"

    files_to_process = [
        "wi29", "dj38", "qa194", "uy734", "zi929", 
        "mu1979", "ca4663", "tz6117", "eg7146", "ei8246"
    ]

    if not os.path.exists(fig_dir):
        os.makedirs(fig_dir)

    for base_name in files_to_process:
        tsp_path = os.path.join(tsp_dir, f"{base_name}.tsp")
        tour_path = os.path.join(tour_dir, f"{base_name}.tour")
        output_svg = os.path.join(fig_dir, f"{base_name}_tour.svg")

        if os.path.exists(tsp_path) and os.path.exists(tour_path):
            coords = load_tsp(tsp_path)
            tour = load_tour(tour_path)
            if coords and tour:
                plot_tour(base_name, coords, tour, output_svg)
        else:
            print(f"! Brak plików dla: {base_name} (Sprawdź czy masz .tsp w assets i .tour w output)")

if __name__ == "__main__":
    main()
