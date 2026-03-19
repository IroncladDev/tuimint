# TUImint

Fedimint TUI wallet

this is a work in progress

## TODO

- Tutorial
- Database separate from redb instance (kv?)
- TUI
- Make it work
- Multiple wallets/clients per path
- Bitcoinmints integration
- Customizable theme

## Tutorial Slides

---

```
 TUImint                         Fedimint 
                                     🬇🬃   
  ▟▀█▀▙  is a  wallet client for 🬁🬀🬖🬂🬂🬢🬁🬀
  ▜ ▀ ▛                           🬋 🬈🬭🬭🬅 🬋
                                    🬋  🬋  

From a high level,  Fedimint can be described like a  bitcoin bank
```

- center everything
- color terms
- highlight keywords
- bottom description can wrap

---

```
Multiple 🬋 Guardian Servers are linked together

                      🬇🬃    
                   🬁🬀🬖🬂🬂🬢🬁🬀 
                   🬋 🬈🬭🬭🬅 🬋 
                     🬋  🬋   

         to form a single  Federation
```

- color the guardian rectangles differently than the fedimint circle
- color terms

---

```
  If some 🬋 Guardians go offline,
the  Federation can still function

                🬇🬃    
             🬁🬀🬖🬂🬂🬢🬁🬀 
             🬋 🬈🬭🬭🬅 🬋 
               🬋  🬋   

This improves reliability and uptime
```

- color the guardian rectangles differently than the fedimint circle
- color terms
- bottom two guardian dots are red, others are green

---

```
To make a decision such as shutting down the  Federation or updating the  Fedimint software, 🬋 Guardians must vote and reach consensus

                     
              🬖🬂🬂🬢  
              🬈🬭🬭🬅  
               x  x   

This reduces the risk of individual bad actors
```

- Green for checks,
- Red for x's

---

```
Under the Gold Standard, people could exchange  cash for its respective value in gold

                         ▄▄▄                     
 ╭─── $10 of Cash ───▶ ▄█████▄ ◀─── $5 of Gold ───╮
  ◀─ $10 of Gold ──────█ █ █────── $5 of Cash ─▶ 
Bob                    ▀▀▀▀▀▀▀                  Alice
```

---

```
 Fedimint allows you to exchange  Bitcoin for  Ecash Notes and vice versa

                            🬇🬃                       
 ╭─── 0.1 BTC ─────────▶ 🬁🬀🬖🬂🬂🬢🬁🬀 ◀── 0.2 BTC of Ecash ─╮
  ◀─ 0.1 BTC of Ecash ──🬋 🬈🬭🬭🬅 🬋──── 0.2 BTC ────────▶ 
Bob                        🬋  🬋                       Alice

All  Ecash Notes are backed by  Bitcoin, making it impossible to mint fraudulent notes backed by nothing
```

- Highlight "Ecash", "Cash" as green
- Highlight "BTC", "Gold" as orange

---


```
Alice and Bob both use the same  Federation with a  Wallet Client

                    🬇🬃                       
                 🬁🬀🬖🬂🬂🬢🬁🬀                        
         ╭────── 🬋 🬈🬭🬭🬅 🬋 ──────╮                      
         │         🬋  🬋         │                        
╭────── Bob ───────╮   ╭───── Alice ──────╮
│ Balance: 0.1 BTC │   │ Balance: 0.2 BTC │
╰──────────────────╯   ╰──────────────────╯   

When they deposit  Bitcoin into the  Federation, their balances are updated
```

---

```
Bob creates an  Ecash Note worth 0.05 Bitcoin and shares it with Alice.
The amount is deducted from his balance.

╭────── Bob ────────╮          ╭───── Alice ──────╮
│ Balance: 0.05 BTC │    ╭──▶? │ Balance: 0.2 BTC │
╰──│────────────────╯    │     ╰──────────────────╯   
   ╰─ 0.05 BTC of Ecash ─╯

Alice redeems the  Ecash Note. Her balance is updated.

╭────── Bob ────────╮          ╭─────── Alice ───────╮
│ Balance: 0.05 BTC │    ╭─────▶  Balance: 0.25 BTC │
╰──│────────────────╯    │     ╰─────────────────────╯   
   ╰─ 0.05 BTC of Ecash ─╯

The actual Bitcoin in the  Federation that Bob deposited doesn't move, creating a private, untraceable transaction.
```

- First portion of the arrow from Bob is green, second portion is grey
- Second portion becomes green when Alice receives it
- Checkmark is green
- Question mark is yellow

---

```
Bob makes an external bitcoin payment. Money moves from the  Federation to the recipient

                                 🬇🬃                       
 ╭──── Initiate 0.1 BTC ────▶ 🬁🬀🬖🬂🬂🬢🬁🬀─── 0.1 BTC sent ──▶
    payment to <address>     🬋 🬈🬭🬭🬅 🬋    to <address>
Bob                             🬋  🬋       

Bob creates a bitcoin address. When someone pays it, the money is sent to it goes into the  Federation and his balance is updated.

                            🬇🬃                       
 ╭─ 1. create address ─▶ 🬁🬀🬖🬂🬂🬢🬁🬀─ 2. <address 1> created ────╮
                        🬋 🬈🬭🬭🬅 🬋                      3. someone pays
Bob                        🬋  🬋 ─ 4. money goes to ──╮     <address 1>         
 ▲                                  Federation       │        │ 
 ╰── 5. Bob's balance updated ───────────────────────┴────────╯
```

- Highlight "Ecash", "Cash" as green
- Highlight "BTC", "Gold" as orange

---

```
Hopefully you'll enjoy making private, untraceable payments with TUImint

           ⠰⣉⠆                  
      ⢎⡱  ⣀⠤⠤⠤⣀  ⢎⡱     
▀█▀ █ █ █⠊▟▀█▀▙⠑█ █▀█ ▀█▀  
 █  █▄█ █⡀▜ ▀ ▛⢀█ █ █  █   
     ⠰⣉⠆ ⠈⠒⠤⠤⠤⠒⠁ ⠰⣉⠆     
         ⡔⢢   ⡔⢢         
         ⠈⠁   ⠈⠁          

To learn more, check out https://fedimint.org

To view this tutorial again, press 't' from the home screen
```

### Keywords

- TUImint | Bold Yellow? | null
- Fedimint | Yellow | 
- Federation | Blue | 
- Guardian | Purple | 🬋
- Client | Purple | 
- ecash note | Green | 
- Bitcoin | Orange | 

- Use emojis with keywords (sun + fedimint) (ecash note + money) (client + wallet) + syntax color highlight
- Show UI examples (alice and bob) sending and pegging in/out


