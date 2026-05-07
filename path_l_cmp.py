import matplotlib.pyplot as plt
import pandas as pd

tsp_sizes = {
    "wi29": 29, "dj38": 38, "qa194": 194, "uy734": 734, "zi929": 929,
    "mu1979": 1979, "ca4663": 4663, "tz6117": 6117, "eg7146": 7146, "ei8246": 8246
}

optimal_results = {
    "wi29": 27603, "dj38": 6656, "qa194": 9352, "uy734": 79114, "zi929": 95345,
    "mu1979": 86891, "ca4663": 1290319, "tz6117": 259045, "eg7146": 501507, "ei8246": 206171
}

ann_results = {
    "ca4663": 1355651.52, "dj38": 6659.43, "eg7146": 181049.50, "ei8246": 219994.85,
    "mu1979": 89769.91, "qa194": 9437.73, "tz6117": 417762.54, "uy734": 81324.27,
    "wi29": 27601.17, "zi929": 97443.77
}

tabu_results = {
    "ca4663": 3980677.91, "dj38": 6659.43, "eg7146": 965296.17, "ei8246": 1479252.09,
    "mu1979": 134567.19, "qa194": 10039.70, "tz6117": 1841657.65, "uy734": 93830.30,
    "wi29": 27601.17, "zi929": 109839.34
}

data = []
for key in tsp_sizes:
    data.append({
        "Rozmiar": tsp_sizes[key],
        "ANN": ann_results[key],
        "TABU": tabu_results[key],
        "Optimal": optimal_results[key]
    })

df = pd.DataFrame(data).sort_values(by="Rozmiar")

plt.figure(figsize=(10, 6))

plt.plot(df["Rozmiar"], df["ANN"], marker='o', label='ANN', color='#1f77b4', linewidth=2)
plt.plot(df["Rozmiar"], df["TABU"], marker='s', label='TABU', color='#ff7f0e', linestyle='--', linewidth=2)

plt.plot(df["Rozmiar"], df["Optimal"], marker='x', label='Optimal (Reference)', color='#2ca02c', linestyle=':', alpha=0.8)

plt.yscale('log')
plt.title("Porównanie długości ścieżek względem wartości optymalnych")
plt.xlabel("Liczba miast")
plt.ylabel("Długość ścieżki (skala logarytmiczna)")
plt.grid(True, which="both", ls="-", alpha=0.3)
plt.legend()

plt.tight_layout()
plt.savefig("path_results.svg")
