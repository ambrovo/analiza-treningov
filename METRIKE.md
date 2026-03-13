# PeakPower - Pregled Metrik

Pregled vseh metrik, ki jih PeakPower izračunava in shranjuje.

---

## METRIKE NA TRENINGIH (Per-Workout)

| Metrika | Opis |
|---------|------|
| **PDC (Power Duration Curve)** | Najboljša povprečna moč na standardnih trajanih (1s do 2h) |
| **Normalized Power (NP)** | Fiziološko prilagojena moč, ki upošteva variabilnost treninga |
| **Intensity Factor (IF)** | Relativna intenziteta glede na FTP (NP/FTP) |
| **Training Stress Score (TSS)** | Obremenitev treninga (1 ura pri FTP = 100 TSS) |
| **Variability Index (VI)** | Mera variabilnosti moči (NP/avg_power) - ali je trening "steady" |
| **Power Zone Distribution** | Čas (sekunde) v vsaki moč coni (Z1-Z7) |
| **Heart Rate Zone Distribution** | Čas (sekunde) v vsaki srčni coni (Z1-Z5) |
| **Severe Domain Seconds** | Sekunde z močjo nad FTP |
| **Extreme Domain Seconds** | Sekunde z močjo nad 150% FTP (nevromuskularna cona) |
| **Total Power Seconds** | Skupno število sekund z veljavnimi podatki o moči |
| **Total Work (kJ)** | Skupno mehaniško delo proizvedeno med treningom |
| **Peak VAM** | Maksimalna hitrost vzpenjanja (m/h) na 5min, 10min, 20min |
| **Fresh PDC** | Power krivulja iz celotnega treninga (najboljša možna moč) |
| **Fatigued PDC** | Power krivulja po akumulaciji dela (1000/2000/3000 kJ) |
| **Fatigue Resistance Drops** | Procentualni padec moči po utrujanju na ključnih trajanih |
| **Fatigue Resistance Index (FRI)** | Razmerje 5min moči po 2000 kJ vs. sveža 5min moč |
| **Aerobic Efficiency (EF)** | Normalizirana moč na srčni utrip (W/bpm) - učinkovitost |
| **Aerobic Decoupling** | Degradacija moč/HR od prve do druge polovice treninga |
| **HR Drift Rate** | Hitrost dviga srčnega utripa skozi čas (bpm/min) |
| **Power-to-HR Slope** | Linearni odnos med močjo in srčnim utripom |
| **Aerobic Quality Score** | Sestavljena metrika kakovosti aerobnega treninga (0.0-1.0) |
| **W' Balance** | Sledenje depleciji anaerobne kapacitete med treningom |
| **W' Recovery** | Najboljša regeneracija W' na 60s in 300s oknih |
| **Power Density Histogram** | Distribucija moči v 10W korakih |
| **HR Density Histogram** | Distribucija srčnega utripa v 5 bpm korakih |
| **Compound Score (Fresh)** | Razmerje moč/teža na 5 minutah (W²/kg) |
| **Compound Score (Fatigued)** | Razmerje moč/teža na 5 minutah PO 2000 kJ dela |
| **Durability Ratio** | Razmerje utrujene vs. sveže moči |
| **Power Coverage** | Odstotek vzorcev z veljavnimi podatki o moči |
| **HR Coverage** | Odstotek vzorcev z veljavnimi HR podatki |
| **Power Spike Count** | Število nerealno visokih moči (napake senzorjev) |
| **HR Dropout Seconds** | Sekunde brez veljavnih HR podatkov |
| **Data Quality Score** | Skupna ocena kakovosti podatkov (0.0-1.0) |
| **Load Axes** | Obremenitev razdeljena na 6 fizioloških osi (aerobna, tempo, threshold, VO2, anaerobna, NM) |
| **Workout Archetype** | Kategorizacija tipa treninga (Endurance, Tempo, Threshold, VO2max, Anaerobic, Mixed) |

---

## METRIKE SKOZI ČAS (Historical/Athlete-Level)

| Metrika | Opis |
|---------|------|
| **CTL (Chronic Training Load)** | Dolgoročna forma - eksponentno ponderirana obremenitev zadnjih 42 dni |
| **ATL (Acute Training Load)** | Kratkoročna utrujenost - eksponentno ponderirana obremenitev zadnjih 7 dni |
| **TSB (Training Stress Balance)** | Pripravljenost - razlika med formo in utrujenostjo (CTL - ATL) |
| **ACWR** | Razmerje 7-dnevne vs. 28-dnevne obremenitve - indikator tveganja za poškodbo |
| **Critical Power Models** | 4 modeli za power-duration razmerje (Monod, Morton, Péronnet-Thibault, OmPD) |
| **Skupna Power Krivulja** | Najboljša moč na vsakem trajanju preko vseh treningov (all-time, 90d, 30d, po športu) |
| **CP Progression** | Časovna serija CP ocen - kako se CP spreminja skozi sezono |

---

## OSNOVNI PODATKI (Potrebni za izračune)

### Iz .fit/.tcx datotek:
- Power, Heart Rate, Cadence, Speed, Altitude, GPS (časovne serije)
- Povprečna/maksimalna moč, povprečen/maksimalen HR, trajanje, razdalja

### Profil športnika:
- FTP (Functional Threshold Power)
- LT1 power (Aerobni prag - moč)
- LTHR (Lactate Threshold Heart Rate)
- LT1 HR (Aerobni prag - srčni utrip)
- Max HR (Maksimalni srčni utrip)
- Teža (kg)

---


