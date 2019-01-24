# UML-Visualisierung - Eine Software zur Visualisierung von textuell spezifizierten UML-Diagrammen

## Getting started
* Nach Start des Programmes kann man direkt loslegen und Eingaben tätigen

## Syntax
### Klasse
* Eine Klasse besteht hier aus mehreren Komponenten:
  1. ID
  Jede Klasse hat hier eine ID, die benutzt wird um Klassen zu identifizieren und einander zuordnen zu können. Wie viele Dinge hier führt eine falsche Eingabe hier zum Überspringen der Klasse.
  2. Typ
  Eine Klasse kann vom Typ *Interface*, *Class* oder *Abstract* sein.
  3. Name
  Der Name der Klasse, hier muss man sich nicht an z.B. Javakonventionen halten, sollte man aber generell.
  4. Attribute
    1. Zugriffsmodifikator *public*, *protected*, *package* oder *private*.
    2. Static, wenn es nicht static ist lässt man dieses Feld leer. Auch __kein__ Leerzeichen.
    3. Final, wenn es nicht final ist lässt man dieses Feld leer. Auch __kein__ Leerzeichen.
    4. Datentyp, kann alles mögliche sein.
    5. Name, kann ebenfalls alles mögliche sein.
   5. Methoden
    1. Zugriffsmodifikator *public*, *protected*, *package* oder *private*.
    2. Static, wenn es nicht static ist lässt man dieses Feld leer. Auch __kein__ Leerzeichen.
    3. Final, wenn es nicht final ist lässt man dieses Feld leer. Auch __kein__ Leerzeichen.
    4. Rückgabetyp, kann alles mögliche sein.
    5. Name, kann ebenfalls alles mögliche sein.
    6. Die Parameter bestehen immer aus zwei Teilen: Datentyp und name, dies schreibt man in der Form Datentyp=name, mehrere Parameter sind durch Leerzeichen getrennt
### Use-Case
* Komplettes Beispiel:UseCase~Systemname;5:Akteur:4:20;20:Case name:EP;
* Ein Use-Case besteht hier aus mehreren Komponenten und diese Komponenten werden mit ";" Semikolon getrennt:
  *Name
  1.Jedes Use-Case hat einen Systemnamen. Der Systemname wird direkt am Anfang angegeben. 
  Bsp:UseCase~Systemname;
  *Akteur
 1.Jedes Use_case hat auch einen oder mehrere Akteure. Es ist möglich jeden Akteur einen Namen zu geben und eine Vererbung zu einem     anderen Akteur zu erstellen. 
  Bsp:5:Akteur:4:20;
  Die 5 sagt an welcher Stelle der Akteur erstellt wird (1-8).
  Dort wo Akteur steht ist der Name von dem Akteur an der Stelle 5.
  Die 4 sagt das dieser Akteur an stelle 5 von dem Akteur an Stelle 4 erbt.
  Die 20 gibt an das dieser Akteur an der Position 5 mit einem Case an der Stelle 20 per assoziation verbunden ist, hinter der 20 kann man mit Komma getrennt weitere Case Assoziations verbindungen erstellen. 
  * Cases
  1.Jedes Use-Case besteht aus einem oder mehreren Cases. Die Cases können mit beschriftung erstellt werden.
  Bsp:20:Case name:EP;
  Die 20 gibt an das, dass Case an der Stelle 20 erstellt wird und das EP bedeutet das es sich um ein Extend Case handelt, wird das EP weggelassen dann wird ein normales Case erstellt.
  * Extend und Include
 1.Es können auch Extend und Include beziehungen zwischen den einzelnen Cases erstellt werden.
  Bsp. für Extend:Extends:12->8:condition {Kühlschrank leer},
  Bsp. für IncludeInclude:20->16:
  Anfangs schreibt man "Extends:" oder "Include:" hin, je nach dem ob es Extend oder Include sein soll.
  Nach dem Doppeltpunkt wird zuerst das Case angegeben von wo aus das Extend oder Include aus gezeichnet wird angegeben und dann folgt ein Pfeil "->", gefolgt von dem Case wohin gezeichnet werden soll. Zum schluss wird dann wenn es ein Extend ist angegeben welche condition dieses Extend hat. Dieses wird folgendermaßen angegeben   condition {Kühlschrank leer}    zuerst schreibt man condition und die die geschweiften Klammern kommt dann der jeweilige Text.
