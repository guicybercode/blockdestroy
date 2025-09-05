# Purple Box Destruction

A purple box destruction game implemented in Rust using the ggez library for graphical rendering.

## Features

- **Box destruction gameplay** featuring classic pieces
- **Modern graphical interface** with vibrant colors and visual effects  
- **Progressive scoring system** based on the number of cleared lines  
- **Automatic difficulty increase** as the level rises  
- **Intuitive controls** with keyboard support  
- **Ghost piece** for better placement visualization  
- **Pause and restart system**  
- **Modular codebase** following Rust best practices  

## Controls

- **Left/Right Arrows**: Move piece horizontally  
- **Down Arrow**: Move piece downward  
- **Up Arrow**: Rotate piece  
- **Spacebar**: Hard drop (instant fall)  
- **P**: Pause/Unpause game  
- **R**: Restart game  
- **ESC**: Exit game  

## Scoring System

- **1 line**: 100 points × level  
- **2 lines**: 300 points × level  
- **3 lines**: 500 points × level  
- **4 lines (Destruction)**: 800 points × level  
- **Hard drop**: 2 points per dropped line  

## Levels

The level increases every 10 cleared lines. Drop speed increases with level, making the game more challenging.

## Installation

### Prerequisites

- Rust (version 1.70 or higher)  
- Cargo  


### Compilação e Execução

1. Clone o repositório:
```bash
git clone <url-do-repositorio>
cd purple-box-destruction
```

2. Compile e execute o jogo:
```bash
cargo run --release
```

### Compilação para Distribuição

```bash
cargo build --release
```

O executável será criado em `target/release/purple-box-destruction`.

## Estrutura do Projeto

```
src/
├── main.rs          # Ponto de entrada e loop principal
├── game.rs          # Lógica principal do jogo
├── tetromino.rs     # Definição das peças e suas rotações
├── board.rs         # Gerenciamento do tabuleiro e colisões
├── ui.rs           # Interface do usuário e elementos visuais
└── audio.rs        # Sistema de áudio (placeholder)
```

## Tecnologias Utilizadas

- **Rust**: Linguagem principal
- **ggez**: Biblioteca de jogos 2D
- **rand**: Geração de números aleatórios
- **serde**: Serialização (para futuras funcionalidades)

## Funcionalidades Futuras

- [ ] Sistema de áudio com música chiptune
- [ ] Efeitos sonoros
- [ ] Sistema de high scores
- [ ] Modos de jogo alternativos
- [ ] Configurações personalizáveis
- [ ] Suporte a gamepad
- [ ] Modo multiplayer local

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para:

1. Reportar bugs
2. Sugerir novas funcionalidades
3. Enviar pull requests
4. Melhorar a documentação

## Licença

Este projeto está licenciado sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.

## :D

하나님의 축복이 있기를 바랍니다 🙏 (God bless you)

