# TUImint

Fedimint TUI wallet

## Slides

- TUIMint - a Fedimint TUI wallet
- TUIs are great for sovereignty because they abide by the principles of FOSS (emphasize on downloading, distributing), and they are a vibe
- TUIMint is a self-custodial fedimint wallet client that runs on your machine (you can also export your keys)

## Outline

- TUIs are better for sovereignty than web-based applications
    - Follows FOSS principles
- Fedimint is great for sovereignty
    - Export and own your own keys

- Slides in neovim buffers

### Functionality

- Setup
    - Join a federation (don't worry about multiple yet)
    - Send/receive ecash (use ascii qr codes)
    - Export keys
    - Rejoin from existing db

#### End Methods

- clientJoin (new)
- clientOpen (exiting)
- send
- receive
- display wallet balance
- reveal mnemonic

#### TUI Screens/outline

- Intro page (title with input for invite code + submit button)
- Wallet page (send/receive buttons, balance)
- Send modal (ecash)
    - Send input (amount input, send button)
    - Send success (QR, copy button)
    - Send error (maybe, message)
- Receive modal (ecash)
    - Input (text)
    - Success (amount received)
    - Error (maybe, text)

Nice-to-have
- lightning
- (maybe) use BDK for converting msats to sats
- (maybe) use open time lib (install it just to troll)

## TODO

- Clean up vibe coded slop and start anew
    - Framerate to support animations
- Clean up Wallet contructor
    - Creation/new logic
- Support multiple federations
- Support switching
- Lightning (maybe)
- QR Codes (maybe)
- Animated Ecash Codes (possibly never)

### To learn

- How to use db
- Ratatui stuff
