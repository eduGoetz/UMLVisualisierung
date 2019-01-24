# UML-Visualisierung - Eine Software zur Visualisierung von textuell spezifizierten UML-Diagrammen

## Getting started
* Nach Start des Programmes kann man direkt loslegen und Eingaben tätigen

## Syntax
### Klasse
#### Eine Klasse besteht hier aus mehreren Komponenten:<br/>
  * ID<br/>
  Jede Klasse hat hier eine ID, die benutzt wird um Klassen zu identifizieren und einander zuordnen zu können. Wie viele Dinge hier führt eine falsche Eingabe hier zum Überspringen der Klasse.<br/>
  * Typ<br/>
  Eine Klasse kann vom Typ *Interface*, *Class* oder *Abstract* sein.<br/>
  * Name<br/>
  Der Name der Klasse, hier muss man sich nicht an z.B. Javakonventionen halten, sollte man aber generell.<br/>
  * Attribute<br/>
    * Zugriffsmodifikator *public*, *protected*, *package* oder *private*.<br/>
    * Static, wenn es nicht static ist lässt man dieses Feld leer. Auch __kein__ Leerzeichen.<br/>
    * Final, wenn es nicht final ist lässt man dieses Feld leer. Auch __kein__ Leerzeichen.<br/>
    * Datentyp, kann alles mögliche sein.<br/>
    * Name, kann ebenfalls alles mögliche sein.<br/>
   * Methoden<br/>
       * Zugriffsmodifikator *public*, *protected*, *package* oder *private*.<br/>
       * Static, wenn es nicht static ist lässt man dieses Feld leer. Auch __kein__ Leerzeichen.<br/>
       * Final, wenn es nicht final ist lässt man dieses Feld leer. Auch __kein__ Leerzeichen.<br/>
       * Rückgabetyp, kann alles mögliche sein.<br/>
       * Name, kann ebenfalls alles mögliche sein.<br/>
       * Die Parameter bestehen immer aus zwei Teilen: Datentyp und name, dies schreibt man in der Form Datentyp=name, mehrere Parameter sind durch Leerzeichen getrennt<br/>
   * Relationen<br/>
       * Relationen können ganz am ende angegeben werden. Wobei die einzelnen Relationen verschiedene Abkürzungen haben.
          * V  = Vererbung
          * AG = Aggregation
          * K  = Komposition
          * A  = Asspziation
          * gA = gerichtete Assoziation 
          * I  = Implementierung
          * AB = abhängigkeit
          
### Use-Case
#### Komplettes Beispiel:UseCase~Systemname;5:Akteur:4:20;20:Case name:EP;Extends:12->8:condition {text},Include:20->16:
#### Ein Use-Case besteht hier aus mehreren Komponenten und diese Komponenten werden mit ";" Semikolon getrennt:
  * Name<br/>
    * Jedes Use-Case hat einen Systemnamen. Der Systemname wird direkt am Anfang angegeben.<br/> 
    * Bsp:UseCase~Systemname;<br/>
  *  Akteur<br/>
     * Jedes Use_case hat auch einen oder mehrere Akteure. Es ist möglich jeden Akteur einen Namen zu geben und eine Vererbung zu einem         anderen Akteur zu erstellen. <br/>
     * Bsp:5:Akteur:4:20;<br/>
     * Die 5 sagt an welcher Stelle der Akteur erstellt wird (1-8).<br/>
     * Dort wo Akteur steht ist der Name von dem Akteur an der Stelle 5.<br/>
     * Die 4 sagt das dieser Akteur an stelle 5 von dem Akteur an Stelle 4 erbt.<br/>
     * Die 20 gibt an das dieser Akteur an der Position 5 mit einem Case an der Stelle 20 per assoziation verbunden ist, hinter der 20        kann man mit Komma getrennt weitere Case Assoziations verbindungen erstellen. <br/>
  * Cases<br/>
    * Jedes Use-Case besteht aus einem oder mehreren Cases. Die Cases können mit beschriftung erstellt werden.<br/>
    * Bsp:20:Case name:EP;<br/>
    * Die 20 gibt an das, dass Case an der Stelle 20 erstellt wird und das EP bedeutet das es sich um ein Extend Case handelt, wird das         EP weggelassen dann wird ein normales Case erstellt.<br/>
  * Extend und Include<br/>
    * Es können auch Extend und Include beziehungen zwischen den einzelnen Cases erstellt werden.<br/>
    * Bsp. für Extend:Extends:12->8:condition {Text},<br/>
    * Bsp. für IncludeInclude:20->16:<br/>
    * Anfangs schreibt man "Extends:" oder "Include:" hin, je nach dem ob es Extend oder Include sein soll.<br/>
    Nach dem Doppeltpunkt wird zuerst das Case angegeben von wo aus das Extend oder Include aus gezeichnet wird angegeben und dann folgt      ein Pfeil "->", gefolgt von dem Case wohin gezeichnet werden soll. Zum schluss wird dann wenn es ein Extend ist angegeben welche        condition dieses Extend hat. Um diese zu Beschiften gibt man nach dem Doppelpunkt einfach den Text an.
