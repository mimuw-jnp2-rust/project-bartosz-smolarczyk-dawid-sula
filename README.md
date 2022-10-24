## Autorzy
- Dawid Sula (@dawidsula26 na githubie)
- Bartosz Smolarczyk (@SmolSir na githubie)

## Opis
Aplikacja analizująca zachowanie cen na rynku, na którym ważnym czynnikiem są koszty transportu. Aplikacja otrzymywałaby: siatkę połączeń pomiędzy różnymi miejscami; koszty transportu towaru między nimi; funkcje popytu i podaży w podanych miejscach. Na ich podstawie ustalałaby jak ukształtują się ceny w różnych miejsach.

## Biblioteki
- serde:
    - obsługa serializacji i deserializacji plików
- serde_json:
    - automatyczna serializacja i deserializacja plików formatu ```JSON``` wykorzystywanych w projekcie
- plotters:
    - tworzenie wykresu przedstawiającego kształtowanie się cen w miarę przebiegu symulacji
    - w razie błędu kompilacji z powodu braku pakietu freetype2 należy go zainstalować poleceniem:
    
        ```
        apt-get install cmake libfreetype6-dev libfontconfig1-dev xclip
        ```
- ordered-float:
    - implementacja reprezentacji funkcji
- rayon, dashmap:
    - zrównoleglanie obliczeń podczas wyliczania cen i przetwarzania funkcji
