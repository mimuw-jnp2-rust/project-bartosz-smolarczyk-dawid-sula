# Nazwa

## Autorzy
- Dawid Sula (@dawidsula26 na githubie)
- Bartosz Smolarczyk (@SmolSir na githubie)

## Opis
Aplikacja analizująca zachowanie cen na rynku, na którym ważnym czynnikiem są koszty transportu. Aplikacja otrzymywałaby: siatkę połączeń pomiędzy różnymi miejscami; koszty transportu towaru między nimi; funkcje popytu i podaży w podanych miejscach. Na ich podstawie ustalałaby jak ukształtują się ceny w różnych miejsach.

## Funkcjonalność
- Podstawowa:
    - wczytywanie funkcji popytu i podaży z pliku
    - wczytywanie opisu połączeń i kosztów
    - liczenie wyniku dla popytu/podaży opisanych jako tablice - O(liczba jednostek dobra na rynku)
    - proste wyświetlanie wyników 

- Dodatkowa:
    - wyświetlanie tras, na których transportowane są dobra

- Rozszerzona (jeżeli będzie chciała działać):
    - liczenie wyniku dla bardziej zwięzłych opisów popytu/podaży - poprawienie działania na rynkach gdzie jest bardzo dużo jednostek.
    - rozwiązywanie podobnego problemu: dodatkowo otrzymujemy koszty produkcji w naszym przedsiębiorstwie i mamy powiedzieć gdzie i ile powinniśmy produkować.

## Propozycja podziału na części
Część pierwsza: zaimplementowanie podstawowych funkcjonalności.

Część druga: równoległość podczas obliczania wyniku.

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
