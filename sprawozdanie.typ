#set page(paper: "a4", numbering: "1")
#set text(font: "New Computer Modern", size: 11pt, lang: "pl")

#align(center)[
  #v(2cm)
  #text(size: 20pt, weight: "bold")[Algorytmy Metaheurystyczne]
  
  #v(1cm)
  #text(size: 16pt)[Algorytm Wyżarzania i Tabu Search]
  
  #v(1cm)
  #text(size: 16pt)[Mateusz Smuga]

  #v(1cm)
  #datetime.today().display("[day].[month].[year]")
  
  #v(2cm)
]

#pagebreak()

= Metody

== Dane
Do wykonania zadania użyto danych ze 
#link("https://www.math.uwaterloo.ca/tsp/world/countries.html")[#text(blue)[strony wydziału matematyki Uniwersytetu Waterloo]].
Wszystkie użyte pliki znajdują się w folderze `assets/`.
W ramach pracy wykorzystano zbiory danych: \
#align(center)[
  #table(
  columns: (auto, auto, auto),
  inset: 10pt,
  [Zbiór Danych], [Liczba Miast], [Nazwa Pliku],
  [Western Sahara], [29], [wi29.tsp],
  [Djibouti], [38], [dj38.tsp],
  [Quatar], [194], [qa194.tsp],
  [Uruguay], [734], [uy734.tsp],
  [Zimbabwe], [929], [zi929.tsp],
  [Oman], [1979], [mu1979.tsp],
  [Canada], [4663], [ca4663.tsp],
  [Tanzania], [6117], [tz6117.tsp],
  [Egypt], [7146], [eg7146.tsp],
  [Ireland], [8246], [ei8246.tsp],
)]

== Otoczenie
Do obu metod dobrano otoczenie 2-optymalne, gdzie rozcinamy dwie krzyżujące się krawędzie i je rozplątujemy
licząc na krótszą całkowitą trasę. Zostało ono zaimplementowane w pliku `math_util.rs` wraz z potrzebnymi funkcjami.

== Metoda Symulowanego Wyżarzania
Metoda wykorzystuje pojęcia temperatury, epoki oraz otoczenia.
Wszystkie omawiane fragmenty kodu w tej sekcji znajdują się w plikach `annealing.rs` i `simulate_annealing.rs`.

=== Temperatura
Temperatura jest funkcją "czasu", która pozwala nam wykonywać nie opłacalne posunięcia,
aby nie zatrzymywać się w maksimum lokalnym. Z upływem czasu spada i "złe" ruchy zdarzają się 
coraz rzadziej. W tej implementacji temperatura początkowa jest dobierana na podstawie rozmiaru danych i pewnej stałej dobieranej przez użytkownika.
```rust
impl SimulationConstants {
    pub fn from(p: &SimulationParameters) -> Self {
        let n = p.size as f64;

        let base_temperature = p.base_temperature_k * n.ln();
```
base_temperature_k – parametr dobierany przed uruchomieniem algorytmu \
n – rozmiar wczytanych danych \

Sama temperatura jest odpowiednio aktualizowana przy użyciu funkcji
`update_temperature` : \
```rust
pub fn update_temperature(
    temperature: &mut f64,
    sim_consts: &SimulationConstants,
) {
    *temperature *= sim_consts.temp_constant.exp();
}
``` \
Uwaga: \
```rust
let tau = p.tau_k * base_temperature;
let temp_constant = (-1.0)/n.ln();
let min_delta = p.min_delta;

``` \

Tę rekurencyjną zależność można opisać wzorem:

$
T(t, n) = cases(
  k dot ln(n) " : " t = 0,
  T(t-1, n) dot e^(-1 / ln(n)) " : " t > 0
) 
$

Przygotowano też wykres porównujący zachowania funkcji temperatury dla różnych rozmiarów danych i standardowo przyjętych stałych:
#align(center)[ #image("figures/decay_comparison.svg")]
=== Epoka 
Epoką nazywamy wszystkie iteracje z tą samą temperaturą. Tutaj liczba iteracji w danej epoce można dostosować następującymi parametrami:
```rust
pub epoch_min: u64,
pub epoch_max: u64,
pub epoch_range: u64,

```
Są one dobierane w następujący sposób:
```rust
let epoch_min = (p.epoch_k * n) as u64;
let epoch_max = (5.0 * p.epoch_k * n) as u64;
let epoch_range = epoch_max - epoch_min;
```
`epoch_k` – parametr dobierany przez użytkownika

=== Warunek Zatrzymania
Warunek zatrzymania jest dość trywialny w porównania z doborem funkcji temperatury,
mianowicie wystarczy, że temperatura spadnie wystarczająco nisko lub przez ostatnie 50 epok nie zauważymy poprawy:

```rust
pub fn should_stop(
    temp: f64,
    avg_delta: f64,
    no_improve_epochs: u64,
    p: &SimulationConstants,
) -> bool {
    temp < 0.001 || no_improve_epochs >= 50
}
```

=== Działanie Algorytmu
Działanie algorytmu można opisać następującymi krokami: \
1. Wylosuj rozwiązanie początkowe $X$ \
2. Wybierz losowe rozwiązanie $X'$ znajdujące się w sąsiedztwie $X$. \
3. Jeśli $X'$ jest lepsze to je przyjmij. W przeciwnym razie wyznacz prawdopodobieństwo 
przyjęcia $X'$ używając wzoru: 
$
exp((f(X)-f(X'))/T)
$
i z dokładnie tym prawdopodobieństwem $X := X'$. \
4. Jeśli nie skończyła się epoka wróć do punktu drugiego. \
5. Zmniejsz temperaturę. \
6. Jeśli nie osiągnięto warunku stopu wróć do punktu drugiego. \

W kodzie wygląda to w taki sposób:
```rust
pub fn run_simulation(cities: &[City], output_path: &str) -> f64 {
    let n = cities.len();
    if n == 0 { return 0.0; }

    let params = SimulationParameters::new(n as u64);
    let constants = SimulationConstants::from(&params);
    let dist_matrix = create_distance_matrix(cities);

    let mut current_path = generate_random_path(n);
    let mut current_distance = calculate_total_distance(&current_path, &dist_matrix);

    let mut best_path = current_path.clone();
    let mut best_distance = current_distance;

    let mut temperature = constants.base_temperature;
    let mut no_improve_epochs = 0;
    let mut total_iterations = 0;

    println!("Starting SA for {} cities. Initial distance: {:.2}", n, current_distance);

    while !should_stop(temperature, current_distance, no_improve_epochs, &constants) {
        let mut improved_in_epoch = false;
        let current_epoch_len = epoch_length(temperature, &constants);

        for _ in 0..current_epoch_len {
            let (i, j) = get_random_indices(n);
            
            let delta = calculate_2opt_delta(&current_path, i, j, &dist_matrix);

            if accept(delta, temperature, &constants) {
                apply_2opt(&mut current_path, i, j);
                current_distance += delta;

                if current_distance < best_distance {
                    best_distance = current_distance;
                    best_path = current_path.clone();
                    improved_in_epoch = true;
                }
            }
            total_iterations += 1;
        }

        if improved_in_epoch {
            no_improve_epochs = 0;
        } else {
            no_improve_epochs += 1;
        }

        update_temperature(&mut temperature, &constants);
    }

    println!("Simulation finished. Best distance: {:.2} (Total iterations: {})", best_distance, total_iterations);

    if let Err(e) = save_tour(&best_path, output_path) {
        eprintln!("Failed to save tour: {}", e);
    }

    best_distance
}
```

=== Dobór Parametrów
Parametry wykorzystywane w metodzie symulowanego wyżarzania zostały dobrane empirycznie przy ręcznym testowaniu,
tak aby wyniki zbiegały do optymalnych rozwiązań w jak najmniejszym czasie. Dobrane stałe:
```rust
impl SimulationParameters {
    pub const fn new(size: u64) -> Self {
        Self {
            size,
            base_temperature_k: 100.0, 
            epoch_k: 500.0,            
            tau_k: 1.0,
            epsilon: 1e-7,            
            min_delta: -1e-9,
        }
    }
}
```
== Metoda Tabu Search
=== Algorytm
Algorytm rozwija idę Local Search poprzez dodanie listy tabu i sprytniejszego przeszukiwania otoczenia.
Działanie algorytmu można opisać następująco: \
+ Losujemy początkowe rozwiązanie
+ Przeszukujemy jego otoczenie i wybieramy najlepsze rozwiązanie, którego nie ma na liście tabu
+ Umieszczamy je na liście i przyjmujemy jako bieżące rozwiązanie
+ Sprawdzamy warunek stopu

W kodzie odpowiada temu metoda z pliku `tabu_search.rs`.
=== Warunek Zatrzymania
Warunkiem zatrzymania jest wykonanie maksymalnej liczby iteracji.
=== Sposób Wyboru Sąsiada
Sąsiada do analizy wybieramy w sposób losowy dobierając dwa różne wierzchołki i próbując wywołać invert.
=== Dobór Parametrów
Parametry dobrane empirycznie przy ręcznym testowaniu:
```rust
let tabu_params = TabuParameters {
  max_iterations: 10000, 
  tabu_tenure: (n as f64 * 0.2) as usize, 
  neighborhood_size: (n * 2).min(500),   
};
```


= Uzyskane Wyniki 
#let tsp_info = (
  "wi29":   (name: "Western Sahara", optimal: 27603,   opt_img: "witour.gif"),
  "dj38":   (name: "Djibouti",       optimal: 6656,    opt_img: "djtour.gif"),
  "qa194":  (name: "Qatar",          optimal: 9352,    opt_img: "qatour.gif"),
  "uy734":  (name: "Uruguay",        optimal: 79114,   opt_img: "uytour.gif"),
  "zi929":  (name: "Zimbabwe",       optimal: 95345,   opt_img: "zitour.gif"),
  "mu1979": (name: "Oman",           optimal: 86891,   opt_img: "mutour.gif"),
  "ca4663": (name: "Canada",         optimal: 1290319, opt_img: "catour.gif"),
  "tz6117": (name: "Tanzania",       optimal: 259045,  opt_img: "tztour.gif"),
  "eg7146": (name: "Egypt",          optimal: 501507,  opt_img: "egtour.gif"),
  "ei8246": (name: "Irlandia",       optimal: 206171,  opt_img: "eitour.gif"),
)

#let ann_results = (
  "ca4663": 1355651.52, "dj38": 6659.43, "eg7146": 181049.50, "ei8246": 219994.85,
  "mu1979": 89769.91, "qa194": 9437.73, "tz6117": 417762.54, "uy734": 81324.27,
  "wi29": 27601.17, "zi929": 97443.77
)

#let tabu_results = (
  "ca4663": 3980677.91, "dj38": 6659.43, "eg7146": 965296.17, "ei8246": 1479252.09,
  "mu1979": 134567.19, "qa194": 10039.70, "tz6117": 1841657.65, "uy734": 93830.30,
  "wi29": 27601.17, "zi929": 109839.34
)

#let get_size(id) = {
  let digits = id.clusters().filter(c => c in "0123456789").join()
  int(digits)
}

#let results_table(algo_title, algo_prefix, results_map) = {
  let sorted_keys = results_map.keys().sorted(key: get_size)
  
  table(
    columns: (1.5fr, 3.5fr, 3.5fr, 2fr, 2fr),
    align: horizon,
    stroke: 0.5pt + gray,
    inset: 8pt,
    [*Instancja (N)*], [*Wyniki (#algo_title)*], [*Wynik Optymalny*], [*Długość znaleziona*], [*Długość optymalna*],
    
    ..for id in sorted_keys {
      let info = tsp_info.at(id)
      let found = results_map.at(id)
      (
        [*#info.name* \ (#id)],
        image("figures/" + algo_prefix + "_" + id + "_tour.svg", height: 75pt),
        image("optimal_source/" + info.opt_img, height: 75pt),
        [#found],
        [#info.optimal],
      )
    }
  )
}
= Wyniki Algorytmu Wyżarzania (Simulated Annealing)
#results_table("ANN", "ann", ann_results)

#pagebreak()

= Wyniki Algorytmu Tabu Search
#results_table("TABU", "tabu", tabu_results)

= Porównanie długości
#image("path_results.svg")
