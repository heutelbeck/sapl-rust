# tokio stream tests

In der Datei `src/main.rs` ab Zeile 34 werden alle Test Funktionen aufgerufen. Hier können nicht gewünschte Funktionen auskommentiert werden.

## Boolean streams
Es gibt zwei Varianten von boolean streams, die mit einer definierten Taktzeit toggeln. Die erste Variante besteht aus den beiden Dateien
* `src/boolean_stream.rs` implementiert das Trait (Interface) Stream mit der Methode `poll_next()`
* `src/delay.rs` implementiert das Trait (Interface) Future mit der Methode `poll()`

Die zweite Variante ist in `src/main.rs` in Zeile 79-81 zu finden. Dort wird ein AtomicBool und das Struct IntervalStream verwendet. Sehr kompakte Lösung.

## Eager Varianten
* Die erste Variante ist eine manuelle Implementierung, zu finden in der Datei `src/combine_eager.rs`. In dem Struct `CombineEager` wird der letzte Wert beider Streams gespeichert. In der Methode `poll_next()` wird in Zeile 62-63 beide Streams abgefragt und das Ergebnis als Tupel in dem match Block verarbeitet. In der Datei `src/main.rs` ist die Anwendung ab Zeile 17 zu finden.
* Die zweite Variante arbeitet stark mit Marcos. Das führt zu deutlich kürzerem Code, es wird viel abstrahiert. Man muss allerdings auch wissen, wie das jeweilige Marko dazu gebracht werden kann, den gewünschten Code zu erzeugen. In der Datei `src/main.rs` ab Zeile 127 ist die Marko Lösung zu finden.

## Lazy Variante
Eine funktionierende Lösung mittels Markos ist mir noch nicht gelungen. Eine Implementierung von Hand ist in `src/combine_lazy.rs` zu finden. Im dazu gehörigen Struct wird wieder der jeweils letzte Zustand gespeichert. In der Methode `poll_next()` wird nun immer zuerst stream a abgefragt. Nur solange dieser true liefert wird auch der stream b abgefragt. Liefert auch der stream b true, gibt die Funktion `poll_next()` ein true weiter.
In der Datei `src/main.rs` ist die Anwendung dieser Implementierung ab Zeile 46 zu finden.
