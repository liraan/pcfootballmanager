# PC Football Manager

**Architettura modulare, statistiche osservate, attributi latenti, psiche, SENZA CA/PA**

---

**Versione:** 7.0  
**Data:** 24 Ottobre 2026  
**Stato:** Revisione definitiva  

---

## Struttura dei Moduli

```
project-root/
├── README.md                           # questo file — indice generale
├── modules/
│   ├── 00-fondamenta.md               # ruolo, pilastri, dottrina tre piani
│   ├── 01-architettura.md             # workspace Rust, dipendenze, determinismo
│   ├── 02-registry.md                 # registry modulare per dominio
│   ├── 03-pack-system.md              # gestione pack locali, validazione, rollback
│   ├── 04-modelli.md                  # modello dati CANONICO (SENZA CA/PA)
│   ├── 05-generator.md                # generatore procedurale del mondo
│   ├── 06-match-engine.md             # motore partita testuale
│   ├── 07-tactics.md                  # ruoli, formazioni, istruzioni
│   ├── 08-training.md                 # allenamento e staff
│   ├── 09-youth.md                    # accademie giovanili complete
│   ├── 10-ingest.md                   # dati professionali, adattatori, legal
│   ├── 11-scouting.md                 # informazione imperfetta, bias
│   ├── 12-market.md                   # mercato, contratti, negoziazione
│   ├── 13-finance.md                  # finanze, FFP, sponsor
│   ├── 14-ui.md                       # UI/UX, skinning, accessibilità
│   ├── 15-assets.md                   # asset pipeline, kit parametrici
│   ├── 16-editor.md                   # editor e modding
│   ├── 17-ai.md                       # intelligenza artificiale
│   ├── 18-world.md                    # mondo, carriera, media
│   ├── 19-statistics.md               # sistema statistico completo
│   ├── 20-psiche.md                   # modulo psicologico completo (v2)
│   ├── 21-quality.md                  # qualità, test, sicurezza
│   ├── 22-roadmap.md                  # roadmap evolutiva
│   ├── 23-contract.md                 # output required, Definition of Done
│   └── 24-glossario.md                # glossario aggiornato
├── archive/
│   ├── Prompt_Master_v5.md
│   └── Prompt_Master_v6.md
└── docs/
    ├── adr/
    ├── legal/
    └── calibration/
```

---

## Tabella di Tracciabilità v5/v6 → v7

| Sezione v5/v6 | Modulo v7 corrispondente | Stato |
|---|---|---|
| §0-3 | 00-fondamenta | Integrato e rafforzato |
| §4-7 | 01-architettura | Rafforzato (nuovi crate) |
| §5-6 | 02-registry, 03-pack | Confermato |
| §8 | 04-modelli | Completamente rivisto (SENZA CA/PA) |
| §9 | 10-ingest | Mantenuto + Calibration Dashboard |
| §10 | 05-generator | Aggiornato senza CA/PA |
| §11 | 11-scouting | Rafforzato con Dottrina Tre Piani |
| §12 | 06-match-engine | Confermato |
| §13-14 | 07-tactics, 08-training | Intatto |
| §15 | 09-youth | Integro + collegamento psiche |
| §16 | 12-market | Aggiornato senza CA/PA |
| §17-18 | 13-finance, 14-ui | Aggiunte statistiche + psiche |
| §19-20 | 15-assets, 16-editor | Hook aggiunti |
| §21-22 | 17-ai, 18-world | Intatto |
| §23 | 19-statistics | Completamente riscritto |
| §24 | 20-psiche, 21-quality | Nuovo modulo + test aggiornati |
| §25 | 22-roadmap | Aggiornato |
| §26-28 | 23-contract | Consolidato |
| Glossario | 24-glossario | Aggiornato |

**Nota:** Le sezioni non elencate sono mantenute integralmente da v5/v6. Questa è una trasformazione, non un rifacimento da zero.

---

## Dottrina dei Tre Piani (CRITICO — sintetizzata)

Violare questa dottrina significa introdurre bug sistemici non rilevabili.

### Piano A — Ground Truth (Mai Esposto)

- Attributi latenti per ruolo, stati fisiologici, stati psicologici latenti, seed
- **Chi legge:** solo `fmr-match` (motore partita) e generatori procedurali
- **Regola:** nessun modulo esterno può leggere il Piano A. Mai. Nemmeno per debug.

### Piano B — Osservazione (Generata dai Dati Grezzi)

- MatchEvent (ogni azione), statistiche aggregate, pagelle (calcolate DA eventi grezzi, MAI da attributi), trend temporali
- **Chi legge:** sistemi statistici, scouting, mercato (parzialmente), UI
- **Regola CRITICA:** le pagelle DEVONO essere calcolate SOLO da eventi grezzi. Mai usano attributi latenti. Altrimenti hai feedback loop divergenti che corrompono tutto.

### Piano C — Stima (Cosa il Giocante Vede)

- Index di Rendimento (IR) finestrato per contesto, ProjectionProfile (previsioni con intervalli di confidenza), attributi "visibili" (derivate da statistiche, aggiornate periodicamente), report di scouting (con bias e limiti dichiarati)
- **Chi legge:** il giocatore, l'IA di mercato, i sistemi decisionali
- **Regola:** il Piano C non influenza MAI il Piano A. Il giocatore vede solo stime, non verità.

### Feedback Proibiti

- ❌ Piano C → Piano A (le stime non modificano la realtà)
- ❌ Piano A → UI diretta (nessuna fuga di verità)
- ❌ Pagelle usano Piano A (circolarità)
- ❌ Modelli statistici leggono latenti (devono usare solo osservati)

---

## Obiettivo di Prodotto

Gestionale calcistico **single-player first**. Profondità FM24 + anima PC Calcio:

- Interfaccia densa e leggible
- Cronaca testuale radiofonica
- Editor potentissimo
- Gestione extragestorale
- Settimanale sportivo integrato
- Esonero reale
- Statistiche complete, mai sovrascritte
- Psiche come sistema di gioco, non decorazione
- Zero CA/PA: tutto osservabile, derivato, finestrato

**Target:**
- Piattaforme: Windows, Linux, macOS
- Risoluzione minima: 1280×720
- Performance: 60 fps UI, stagione completa < 60s su hardware medio
- Storage: < 5 GB base

---

## Pilastri di Design

1. **Determinismo tracciabile**: stesso seed + stessi input = stesso risultato. Mai compromesso.

2. **Separazione netta dati/logica/UI**: le regole non vivono nell'interfaccia. Le UI sono view-only.

3. **Moddabilità di primo livello**: pack locali, UI integrata, validazione, rollback.

4. **Leggibilità prima della grafica**: densità informativa regolabile, tooltip ovunque, zero colori hardcoded.

5. **Trasparenza radicale**: ogni evento ha spiegazione consultabile. "Perché ho perso?", "Perché quel ragazzo non cresce?", "Perché la valutazione è cambiata?" devono avere risposta.

6. **Il settore giovanile è un gioco dentro il gioco**: non una schermata secondaria. Sistema completo con economia, dinamiche umane, percorsi.

7. **L'informazione è scarsa e costosa**: lo scouting ha limiti, bias, costi. Non è onniscienza.

8. **Moduli viventi**: espandi, contra, versiona, sostituisci senza rompere gli altri. I contratti sono stabili, le implementazioni no.

9. **Nessun CA/PA**: eliminati come scalari. Sostituiti da index osservati + profili di proiezione con confidenza.

10. **Piano B ≠ Piano A**: le statistiche sono osservazioni, non verità. Il gioco insegna al giocatore a distinguere le due cose.

---

## Come Usare Questa Documentazione

1. **Leggi 00-fondamenta.md** — capisci la filosofia e i vincoli irrinunciabili
2. **Leggi 01-architettura.md** — comprendi la struttura del progetto e le regole di dipendenza
3. **Approfondisci per dominio** — ogni modulo è autoportante, puoi saltare dove vuoi
4. **Consulta 23-contract.md** — per capire cosa devi produrre quando implementi una feature
5. **Verifica 24-glossario.md** — termini tecnici definiti una volta per tutte

**Attenzione:** 04-modelli.md, 19-statistics.md e 20-psiche.md sono i tre moduli più critici. Cambiano tutto il resto. Leggili per primi dopo le fondamenta.

---

## Avvertenze Preliminari

### Cosa è cambiato rispetto a v5/v6

| Elemento | v5/v6 | v7 | Motivazione |
|---|---|---|---|
| **CA/PA** | Esistenti, derivati | ELIMINATI completamente | Elimina circolarità, falsa certezza, arbitrio |
| **Valutazione** | Scalare CA 50-200 | Index di Rendimento finestrato + Attributi Latenti per ruolo | Separazione capacità vs produzione |
| **Proiezioni** | PA come tetto fisso | ProjectionProfile con banda di confidenza | Onestà epistemica: nessuno sa il futuro |
| **Psiche** | Accennata (§11, §15.8) | Modulo completo a tre livelli, eventi, interventi | Feature completa, non aggiunta |
| **Documentazione** | Diff con riferimenti incrociati | Moduli autoportanti con tracciabilità | Manutenibilità a lungo termine |
| **Tre Piani** | Non specificati | Dottrina esplicita (Ground Truth/Osservazione/Stima) | Previene leak di informazioni, fallacie cognitive |

### Regole di Navigazione

- Ogni modulo **deve essere autoportante** — non fare riferimento ad "altri capitoli", cita il modulo corrispondente
- Ogni modulo ha **versione**, **date**, **dipendenze**, **dipendenti**
- Le sezioni "Divieti" e "Definition of Done" sono obbligatorie in ogni modulo
- Usa tabelle per confronti, diagrammi testuali per flussi
- Niente riempitivi, niente filosofia senza specifica tecnica

