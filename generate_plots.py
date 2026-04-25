import os
import matplotlib.pyplot as plt

def load_tsp(filepath):
    coords = {}
    with open(filepath, 'r') as f:
        section = False
        for line in f:
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
    return coords

def load_tour(filepath):
    tour = []
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
    return tour

def plot_tour(algo_name, instance_name, coords, tour, output_path):
    plt.figure(figsize=(10, 10))
    path_nodes = tour + [tour[0]]
    
    c1 = [coords[node][0] for node in path_nodes] # Lat (zazwyczaj Y)
    c2 = [coords[node][1] for node in path_nodes] # Lon (zazwyczaj X)

    # Transformacje współrzędnych dla poprawnej orientacji map
# Transformacje współrzędnych dla poprawnej orientacji map
    if any(k in instance_name for k in ["ca4663", "wi29", "ei8246"]):
        # Odbicie względem osi Y (Irlandia, Kanada, Sahara)
        x, y = [-v for v in c2], c1
    elif "tz6117" in instance_name:
        # Odbicie względem osi X (Tanzania)
        x, y = c2, [-v for v in c1]
    elif "zi929" in instance_name:
        # Zimbabwe (według Twojego kodu też ma negację c1, czyli osi X)
        x, y = c2, [-v for v in c1]
    elif "uy734" in instance_name:
        # Urugwaj (podwójne odbicie / obrót)
        x, y = [-v for v in c2], [-v for v in c1]
    else:
        # Reszta świata (Egipt, Katar, Dżibuti, Oman)
        x, y = c2, c1

    plt.plot(x, y, color='#2c3e50', linewidth=0.8, alpha=0.7)
    plt.scatter(x, y, color='#e74c3c', s=2)
    plt.gca().set_aspect('equal', adjustable='box')
    plt.axis('off')
    
    plt.title(f"TSP {algo_name}: {instance_name}")
    plt.savefig(output_path, format='svg', bbox_inches='tight')
    plt.close()
def main():
    tsp_dir = "assets"
    tour_dir = "output"
    fig_dir = "figures"
    
    if not os.path.exists(fig_dir):
        os.makedirs(fig_dir)

    for tour_filename in os.listdir(tour_dir):
        if not tour_filename.endswith(".tour"):
            continue
            
        parts = tour_filename.replace(".tour", "").split('_')
        
        if len(parts) >= 3:
            algo_part = parts[1]        
            instance_name = parts[2]    
            
            tsp_path = os.path.join(tsp_dir, f"{instance_name}.tsp")
            tour_path = os.path.join(tour_dir, tour_filename)
            
            output_filename = f"{algo_part.lower()}_{instance_name}_tour.svg"
            output_path = os.path.join(fig_dir, output_filename)
            
            if os.path.exists(tsp_path):
                print(f"Generowanie {output_filename}...")
                coords = load_tsp(tsp_path)
                tour = load_tour(tour_path)
                if coords and tour:
                    plot_tour(algo_part, instance_name, coords, tour, output_path)
            else:
                print(f"Brak pliku TSP dla: {instance_name}")

if __name__ == "__main__":
    main()
