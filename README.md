# experiment-wasm-runtime

Experiment mit WASM

## Ausgangslage + Problem(e)

Inexor soll erweiterbar sein. Die "Zentrale" bildet das Entity System. Es soll darüber hinaus
möglich sein, eine Skriptsprache zum Erweitern und Modden zu verwenden. Beispielsweise könnten
Game-Modes oder Anti-Cheats dynamisch nachgeliefert werden, ohne, dass die Inexor-Rust-Binary
ausgetauscht werden müsste. (oder ein Entity-Behaviour)

Bisher (C++) wurde versucht die Google V8 Engine einzubetten. Lua kommt aus diversen Gründen
nicht als (zumindest alleinige) Skriptsprache infrage. JavaScript schien geeigneter. Erstens
wegen der Mächtigkeit der Sprache im Vergleich zu LUA. Zweitens wegen der Kenntnis vieler
Entwickler von der Sprache. Drittens, weil Skriptsprache für die Engine und Sprache für das
User Interface dieselbe wäre. Das sind mehrere gute Gründe, doch musste leidlich die Erfahrung
gemacht werden, dass das Einbetten von V8 nicht trivial ist und Nachteile zustande kommen, die
mit Lua nicht vorhanden waren. Lua ist hingegen ist leichtgewichtig und verhältnismäßig einfach
einzubinden. Bei V8 ist das Gegenteil der Fall. Die Binärgröße der Executable steigt deutlich.
Die Bibliothek benötigt lange zum Kompilieren und setzt auf einen relativ exotischen
Build-Stack. Die Integration in die Engine ist aufwändig und potenziell fehleranfällig
insbesondere was Speichermanagement und Multithreading betrifft. Außerdem sinkt die Portabilität.

In letzter Zeit tut sich jedoch eine Alternative auf.

## Lösung

WASM wird voraussichtlich der neue heiße Scheiß im Webbrowser. Nicht ganz neu werden Programme
zu Bytecode kompiliert, die von einer Runtime ausgeführt werden. Diese Runtime ist gleichzeitig
eine Sandbox. Ähnlich wie die JVM können verschiedene Sprachen zu WASM-Bytecode kompiliert
werden. Ziel von WASM im Browser ist es schneller zu laden und schneller auszuführen. Eine
WASM-Runtime ist aber nicht auf den Webbrowser beschränkt. Und genau hier wird es interessant,
denn Inexor könnte eine WASM-Runtime einbinden und wäre in der Lage, Module dynamisch zu laden
und auszuführen. Die Runtime könnte mit dem Entity-System gekoppelt werden, sodass man
mithilfe dieser Module das Entity-System steuern kann. So könnten beispielsweise Game-Modes,
aufwändige Map-Logiken, Anti-Cheats, Mapper-Tools, Entity-System-Erweiterungen wie ein neues
Partikel-Emitter-Verhalten usw. ganz klassisch als Modul bereitgestellt werden. Welche
Ausgangssprache für das Modul verwendet wurde, ist dabei zweitrangig. Bonus ist sicherlich,
dass man Rust und Go nativ zu WebAssembly kompilieren kann.

## Ziele / Investigations

### Teil 1

- [x] Ist es prinzipiell machbar?
  - → Ja
- [x] Ist es aufwändig ausführbare Module zu produzieren?
  - → Nein, wer Rust kann und wer schon mal eine Rust-Library erstellt hat, kann es sich prinzipiell mit sehr wenig Aufwand aneignen
- [x] Wie schnell kompilieren solche Module?
  - → Wenn mit Rust, ist es die normale Rust-Geschwindigkeit
  - → Bei anderen Programmiersprachen wird es ähnlich sein, sie werden in ihrer normalen Geschwindigkeit kompiliert
- [x] Was wird benötigt, um solche Module zu kompilieren?
  - → Für Rust gilt: nichts Besonderes. Man kann ganz normal den Paketmanager Cargo verwenden und mit Rustup kann man die notwendigen Build-Target-Typen installieren
- [x] Wie groß sind solche Module?
  - → Sehr klein(!), aber nur, wenn optimiert (release build)
- [x] Wie schnell sind solche Module?
  - → Sehr schnell(!) - man muss jedoch beachten, ob man eher nach Größe oder nach Geschwindigkeit optimieren möchte
- [x] Wie sieht ein WASM-Modul in Rust aus?
  - → Binary (also mit main-Funktion)  → `1-compile-rust-to-wasm`
  - → Library (also mit exportierten Funktionsnamen) → `3-rust-wasm-library`
- [x] Wie kann man Funktionalitäten des Host-Systems (also des Entity-Systems) verwenden und kann man von den WASM-Modulen vernünftig auf eine API zugreifen, insbesondere auf die Entity-System-API?
  - → Das Hostsystem exportiert Funktionen, die in einem WASM-Modul importiert werden können → `4-use-host-functionality`

### Teil 2

- [ ] Kann man tatsächlich verschiedene Sprachen verwenden?
- [ ] Wie sieht ein WASM-Modul in JavaScript oder TypeScript aus?
- [ ] Wie müsste ein WASM-Modul mit dem Entity-System interagieren können?
  * Entity/Relation Instance Generatoren
  * Flow Provider
  * Entity/Relation Behaviour
  * Bräuchte es nach der Integration keine Bibliothek mehr für Expression Handling?
- [ ] Kann man Performance-intensive Entity-Behaviours mit WASM realisieren?
  * Procedural-Texture-Generation / -Manipulation
  * Particle-Effect
  * Collision-Calculation
- [ ] Kann man mithilfe von WASM im Behaviours (Entity-/Relation-Behaviours) multi-threaded einbinden (das WASM-Programm läuft z.B. dauerhaft in einem eigenen Event-Loop)?
  * Scheduler
  * Timer
  * Generatoren
  * Emitter (z.B. Particle Emitter)

### Teil 3

- [ ] Wäre es prinzipiell möglich mithilfe von Emscripten den Vulcan-Renderer innerhalb der WASM-Runtime laufen zu lassen?
- [ ] Kann man die Einfachheit von Skriptsprachen erreichen, indem man zur Laufzeit den Quelltext nach WASM-ByteCode kompiliert?

## Anhang 1: WAT-Beispiel

Ein EntityType `SinnDesLebens` könnte beispielsweise in Web-Assembly implementiert werden.

```wat
(module
  (func (export "answer") (result i32)
    i32.const 42
  )
)
```

## Anhang 2: Gegenüberstellung + / -

| Was                         | V8           | WASM              | Gewichtung |
|-----------------------------|--------------|-------------------|------------|
| Kompilieren notwendig?      | Nein (++)    | Ja, außer WAT (-) | Mittel     |
| Ausführungsgeschwindigkeit  | +            | ++                | Hoch       |
| Sandbox                     | ++           | ++                | Hoch       |
| Compile Time                | --           | +                 | Hoch       |
| Build Stack                 | Exotisch (-) | Nativ (+)         | Hoch       |
| Portabilität                | geringer (-) | höher (++)        | Hoch       |
| Einfluss auf die Binärgröße | hoch (-)     | sehr gering (++)  | Mittel     |

## Anhang 3: Sprachunterstützung

| Sprache    | V8           | WASM |
|------------|--------------|------|
| JavaScript | Ja           | Ja   |
| TypeScript | Ja           | Ja   |
| Rust       | Nein         | Ja   |
| Python     | Nein         | Ja   |
| Lua        | Nein         | ?    |

## Anhang 4: Abgrenzung Plugin-System zu WASM Runtime

In Inexor Reactive Graph Flow gibt es bereits ein Plugin-System. Dieses ist erstens
für die grundlegenden Funktionalitäten gedacht. Auch die WASM-Runtime wird als
Plugin bereitgestellt. Plugins haben außerdem vollen Zugriff, weil sie nicht in
einer Sandbox laufen. Das bedeutet, dass nur Plugins eine Rust-MQTT-Bibliothek
nutzen können. Innerhalb der Sandbox in der WASM-Runtime hat man nur Zugriff auf
das Entity-System.

|                                              | Plugin | WASM |
|----------------------------------------------|--------|------|
| Behaviours (Entity, Relation)                | Ja     | Ja   |
| Provider (Component, Entity, Relation, Flow) | Ja     | Ja   |
| Sandbox                                      | Nein   | Ja   |
| Nutzung von (nativen) Bibliotheken           | Ja     | Nein |

## Lektüre

* https://archium.org/index.php/Der_Go-Webassembly-Spicker
* https://ichi.pro/de/webassembly-module-in-rust-eine-einfuhrung-93822362592258
* https://wasmtime.dev/
  * https://docs.wasmtime.dev/lang-rust.html
* https://www.heise.de/news/State-of-WebAssembly-2021-Beliebteste-Sprache-fuer-Wasm-Anwendungen-ist-Rust-6115317.html
* https://surma.dev/things/js-to-asc/
