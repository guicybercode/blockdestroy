# Purple Box Destruction

Um jogo de destruição de caixas roxas implementado em Rust usando a biblioteca ggez para renderização gráfica.

## Características

- **Jogo de destruição de caixas** com todas as 7 peças clássicas (I, O, T, S, Z, J, L)
- **Interface gráfica moderna** com cores vibrantes e efeitos visuais
- **Sistema de pontuação** progressivo baseado no número de linhas limpas
- **Aumento de dificuldade** automático conforme o nível aumenta
- **Controles intuitivos** com suporte a teclado
- **Peça fantasma** para melhor visualização do posicionamento
- **Sistema de pausa** e reinício
- **Código modular** seguindo as melhores práticas do Rust

## Controles

- **Setas Esquerda/Direita**: Mover peça horizontalmente
- **Seta para Baixo**: Mover peça para baixo
- **Seta para Cima**: Rotacionar peça
- **Espaço**: Hard drop (queda instantânea)
- **P**: Pausar/Despausar jogo
- **R**: Reiniciar jogo
- **ESC**: Sair do jogo

## Sistema de Pontuação

- **1 linha**: 100 pontos × nível
- **2 linhas**: 300 pontos × nível
- **3 linhas**: 500 pontos × nível
- **4 linhas (Destruição)**: 800 pontos × nível
- **Hard drop**: 2 pontos por linha descida

## Níveis

O nível aumenta a cada 10 linhas limpas. A velocidade de queda aumenta com o nível, tornando o jogo mais desafiador.

## Instalação

### Pré-requisitos

- Rust (versão 1.70 ou superior)
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

## Créditos

Desenvolvido como um projeto educacional para demonstrar as capacidades do Rust na criação de jogos 2D.

