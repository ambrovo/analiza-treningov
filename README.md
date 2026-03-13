#Orodje za analizo treningov

## Cilj projekta

Cilj projekta je narediti "orodje" v rustu za analizo .FIT ali .FIT.gz datotek. 

.FIT datoteko dobimo pri izvozu športnih aktivnosti iz aplikacij kot sta Garmin Connect in Tranining Peaks. Datoteke hranijo podatke za vsako sekundo aktivnosti. Obravnavala bova primarno kolesarske aktivnosti, primeri podatkov so: srčni utrip, moč, kadneca (število obratov na sekundo), lokacija. 

Trenerji potrebujejo za analizo preteklih in planiranje prihodnjih treningov podatke o daljši zgodovini aktivnosti (1-4 leta) kar hitro nanese 250-1000 datotek in pri športnikih ki opravijo 500-1000h letno je to 2-4M sekund letno oziroma 2-16M sekund (za vsako sekundo pa imamo podatke ki nas zanimajo). 

trenutno dostopni programi potrebujejo res ogromno časa za obdelavo podatkov, zato bi določil dva cilja projekta: 
1. Čim hitrejša prva obdelava podatkov
2. "sistem" hranjenja že obdelanih podatkov, ki se lahko večkrat uporabijo. 

Točne metrike (ang metrics), ki nas zanimajo in kako se izračunaj, se nahajajo v  [Metrike](METRIKE.md), ki se bodo dopolnjevale tekom projekta. Nekatere nas zanimajo na nivoju ene aktivnosti, druge pa za vso zgodovino.

Ponastavitve za kasnejše (ne prve) analize so odvisne od metrik samih, naprimer: Zanima nas skupen čas v določenih trenažnih conah (1-6 po moči, 1-5 po utripu), cone se pa skozi čas spreminjajo (če športnik napreduje so starejše cone po moči nižje od novejših). Ko se športniku določijo nove cone, je potrebno aktivnosti od zadnje določitve naprej obdelati po novih conah. 


## Namen projekta

Projekt bi (Ambrož) rad vključil v spletno aplikacijo, ki jo razvija. Zato bi rad, da se ga da vključit v backend (Node.js + Express). Če se ne motim se za cilj 2. lahko uporabi isto bazo podatkov (oz samostojno za predstavitev projekta).Tudi za predstavitev podatkov v praksi bi uporabljal Angular frontend. Za potrebe predstavitve projekta lahko narediva  grafe, bi pa raje če lahko to naredim v angularju, tako da se bo dalo povezat v moj projekt. Sklepam, da brez tega dela je projekt dovolj zahteven, je pa tudi težje narediti predstavitev brez prikaza kaj računa najin program. 