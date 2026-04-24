import os
import math

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

def calculate_distance(c1, c2):
    return math.sqrt((c1[0] - c2[0])**2 + (c1[1] - c2[1])**2)

def calculate_total_length(tour, coords):
    if not tour or not coords:
        return 0
    
    total_length = 0
    for i in range(len(tour) - 1):
        total_length += calculate_distance(coords[tour[i]], coords[tour[i+1]])
    
    total_length += calculate_distance(coords[tour[-1]], coords[tour[0]])
    
    return total_length

def main():
    tsp_dir = "assets"
    tour_dir = "output"
    
    print(f"{'Algorytm':<10} | {'Instancja':<10} | {'Długość trasy':<15}")
    print("-" * 45)

    results = []

    for tour_filename in sorted(os.listdir(tour_dir)):
        if not tour_filename.endswith(".tour"):
            continue
            
        parts = tour_filename.replace(".tour", "").split('_')
        
        if len(parts) >= 3:
            algo = parts[1]
            instance = parts[2]
            
            tsp_path = os.path.join(tsp_dir, f"{instance}.tsp")
            tour_path = os.path.join(tour_dir, tour_filename)
            
            if os.path.exists(tsp_path):
                coords = load_tsp(tsp_path)
                tour = load_tour(tour_path)
                
                if coords and tour:
                    length = calculate_total_length(tour, coords)
                    results.append((algo, instance, length))
                    print(f"{algo:<10} | {instance:<10} | {length:>15.2f}")
            else:
                print(f"Brak pliku TSP dla: {instance}")

    with open("wyniki_dlugosci.txt", "w") as f:
        f.write("Algorytm,Instancja,Dlugosc\n")
        for r in results:
            f.write(f"{r[0]},{r[1]},{r[2]:.2f}\n")

if __name__ == "__main__":
    main()
