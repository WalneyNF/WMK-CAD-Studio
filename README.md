<p align="center"><img src="assets/wmk/logo-wmk-cad.jpg" alt="Logo WMK CAD Studio" width="180"></p>

# WMK CAD Studio

**Editor CAD 2D em Rust, com desenho técnico, medições, arquivos DWG/DXF e automação.**

**Personalização e manutenção do fork WMK:** Walney Moreira Klein — walneyk@hotmail.com

> Projeto derivado do [CADCraft](https://github.com/storytold/cadcraft), desenvolvido pela equipe ArtCraft. **Pré-alfa / experimental**: não é um substituto validado do AutoCAD para produção profissional. Preservar atribuições, licenças e avisos legais dos autores originais.

## Para que serve

O WMK CAD Studio é a ferramenta de **desenho técnico 2D** da suíte WMK. Ele permite criar ou editar plantas baixas, desenhos mecânicos, detalhes de peças, esquemas, cotas e layouts. O arquivo de desenho preserva entidades geométricas editáveis, diferentemente de uma imagem JPG/PNG, que contém apenas pixels.

- **Desenhar:** linhas, polilinhas, arcos, círculos, retângulos, polígonos, elipses, splines, textos.
- **Modificar:** mover, copiar, rotacionar, espelhar, offset, trim, extend, fillet, chamfer, matrizes e outras operações.
- **Precisão:** coordenadas, snap a objetos, grade, modo ortogonal, rastreamento polar, restrições e medidas.
- **Organização:** layers (camadas), propriedades, blocos, atributos e hachuras.
- **Documentos:** cotas, textos, líderes, tabelas, layouts, viewports e impressão/plotagem para PDF.
- **Intercâmbio:** DXF e DWG (com limitações importantes), exportação SVG/PNG.
- **Automação:** linha de comando, scripts, JSON, CLI e servidor MCP para integração com agentes.
- **Plataformas:** aplicativo nativo via Rust/egui e aplicação web via WebAssembly.
- **Idioma:** catálogo de interface inclui português brasileiro; verificar textos ainda não traduzidos.

### É concorrente do AutoCAD?

**Em objetivo, sim; em maturidade, não.** CADCraft pretende atender ao fluxo de trabalho do AutoCAD, mas está em **pré-alfa**. No `ROADMAP.md` original (revisão em 2026-10-10), a prontidão estimada para trabalho real é de aproximadamente **38%**. Existem limitações de precisão em certas operações, edição de blocos, fidelidade DWG, plotagem e modelo 3D. Evite usar o software como única fonte de arquivos críticos sem validar os resultados.

## Aprender a usar — primeiro exercício

O repositório inclui o exemplo `examples/apartment.dxf`, uma planta de apartamento 2D.

1. Abra o programa e carregue o arquivo `examples/apartment.dxf`.
2. Use o botão de zoom e a roda do mouse para navegar; arraste o botão do meio para deslocar a vista.
3. Ative/desative **camadas** para entender a separação de paredes, mobiliário e anotações.
4. Selecione linhas para consultar as **propriedades** e medidas.
5. Em um arquivo de teste, desenhe uma linha, depois aplique os comandos `OFFSET`, `TRIM` e `DIM`.
6. Salve como DXF e reabra uma cópia para conferir as entidades.

No prompt do CAD, exemplos de comandos:

```text
line 0,0 @10,0 @0,5 c
circle 5,2 1
zoom e
```

**Atenção:** as unidades do CAD dependem das configurações do desenho; não presuma metros ou milímetros sem conferir.

## Manuais HTML em português (offline)

O ZIP Portable do WMK inclui manuais HTML que abrem com duplo clique, sem precisar instalar nada:

- [Biblioteca de manuais](manuais/index.html) — início, navegação e índice.
- [Primeiro contato](manuais/primeiro-contato.html) — janela, abrir desenho, zoom, pan, seleção e salvamento.
- [Ferramentas](manuais/ferramentas.html) — comandos LINE, PLINE, OFFSET, TRIM, MOVE, cotas, camadas, blocos.
- [Criar uma planta baixa](manuais/planta-baixa.html) — tutorial passo a passo de um quarto 4 × 3 m.
- [Formatos DXF/DWG/PDF/PNG](manuais/arquivos.html) — quais formatos são editáveis e quais são apenas imagens.
- [Resolver problemas](manuais/solucao-problemas.html) — como identificar versão antiga, logs, unidades e build.

### Exemplos práticos

| Exemplo | Arquivo | O que você aprende |
|---|---|---|
| Apartamento | [`examples/apartment.dxf`](examples/apartment.dxf) | Paredes, cotas, texto, camadas, móveis, blocos e hachuras |
| Desenho mecânico | [Manual de ferramentas](manuais/ferramentas.html) | Aprender círculos, furos, espelhamento e cotas — exercício a criar no programa |
| Quarto WMK 4 × 3 m | [Manual de planta baixa](manuais/planta-baixa.html) | Criar o contorno com coordenadas e salvar DXF |

**Atenção:** imagens JPG/PNG/PDF ainda não são importadas como fundos editáveis nesta compilação. O projeto já exporta PNG/PDF. Conversão de imagens em linhas e ligação automática com BIM ainda estão em desenvolvimento, separadamente das ferramentas CAD 2D existentes.

## Compilar no Windows

Requer Rust recente, com toolchain MSVC e dependências de compilação compatíveis. O README original descreve Rust 1.95+ para desktop/web.

```powershell
git clone https://github.com/WalneyNF/WMK-CAD-Studio.git
cd WMK-CAD-Studio
cargo run --release -p cadcraft -- --sample
```

**Nome do pacote Rust:** `cadcraft` (preservado para compatibilidade). **Nome do aplicativo e do executável Portable:** WMK CAD Studio / `WMK-CAD-Studio.exe`. A janela, tela Sobre e o recurso de ícone utilizam a identidade WMK; os autores originais permanecem creditados. Algumas referências técnicas internas permanecem como `cadcraft` para permitir atualizações da base.

### Compilar pelo GitHub Actions

O fork traz um workflow original de release em `.github/workflows/release.yml` que utiliza ações externas da equipe original e parâmetros de assinatura. Para um teste independente, este fork também oferece o workflow **WMK Windows Portable**, que tenta compilar um EXE sem instalador e publica o artefato do job se a compilação passar.

Veja [Actions](https://github.com/WalneyNF/WMK-CAD-Studio/actions) e [Releases públicas](https://github.com/WalneyNF/WMK-CAD-Studio/releases).

O workflow WMK compila e empacota **`WMK-CAD-Studio.exe`**, incluindo logo e metadados WMK. Quando o build e a publicação terminarem com sucesso, o ZIP ficará disponível na aba Releases, sem precisar entrar na área de Actions. A versão disponibilizada inicialmente é uma **prévia experimental**, não um produto profissional homologado.

## Trabalhar com plantas JPG, PNG e PDF — importador WMK

O WMK CAD Studio abre diretamente desenhos vetoriais **DXF/DWG**; o recurso de **anexar imagens JPG/PNG/PDF dentro da área do CAD** ainda não está concluído neste executável.

Para converter uma planta raster em linhas **CAD editáveis**, agora existe um aplicativo auxiliar dentro do projeto irmão **[WMK Architecture Studio / importador](https://github.com/WalneyNF/WMK-Architecture-Studio/tree/main/importador)**. É uma ferramenta experimental de reconhecimento em Python, com **logo WMK, revisão visual das sugestões, calibração de escala por dois pontos e exportação DXF**.

**Exemplo prático:**

1. Baixe o [importador WMK v0.2](https://github.com/WalneyNF/WMK-Architecture-Studio/tree/main/importador) e rode o `iniciar-windows.bat` com Python 3.11+ instalado; alternativamente verifique a [compilação Windows do importador](https://github.com/WalneyNF/WMK-Architecture-Studio/actions).
2. Abra a imagem da planta, clique em **Reconhecer linhas** e examine as linhas verdes geradas sobre a imagem.
3. Clique nos segmentos que são móveis/cotas para **excluí-los** (ficam vermelhos); adicione manualmente linhas não detectadas.
4. Se souber a dimensão de uma parede, use **Calibrar escala** para definir milímetros por pixel com dois pontos.
5. Clique em **Salvar DXF revisado**, depois abra o arquivo no **WMK CAD Studio** e continue a edição.

**Atenção:** a conversão não cria automaticamente portas ou janelas BIM, e a qualidade depende da planta. O DXF não terá escala métrica real enquanto o usuário não calibrar. A imagem original ainda não é anexada ao arquivo DXF do editor.

## Relação com os outros projetos WMK

| Projeto | Responsabilidade |
|---|---|
| [WMK CAD Studio](https://github.com/WalneyNF/WMK-CAD-Studio) | Desenho 2D preciso e DXF/DWG |
| [WMK BIM Studio](https://github.com/WalneyNF/WMK-BIM-STUDIO) | Modelos BIM: paredes, portas, janelas, pisos, telhados, IFC |
| [WMK Architecture Studio](https://github.com/WalneyNF/WMK-Architecture-Studio) | Ponte CAD → BIM, validação geométrica, futura integração OCR/IA |

**Primeira integração já preparada:** no WMK Architecture Studio existe um conversor experimental de linhas DXF, na camada `WMK_PAREDES`, para comandos `wall` do WMK BIM Studio. Ainda não existe sincronização automática ou reconhecimento completo de imagem.

## Identidade visual

O logo aprovado WMK CAD Studio está em [`assets/wmk/logo-wmk-cad.jpg`](assets/wmk/logo-wmk-cad.jpg), em versão reduzida para a interface e documentação. A aplicação tenta incorporá-lo como ícone da janela e recurso Windows durante a compilação. Não distribuir marcas originais ArtCraft como identidade do fork; manter as atribuições originais e licenças.

## Documentação, autoria e licenças

- [README original em inglês](README.original.en.md)
- [Roadmap original](ROADMAP.md)
- [Documentação técnica](docs/)
- [Atribuições](ATTRIBUTION.md), [avisos legais](NOTICE), [MIT](LICENSE-MIT) e [Apache-2.0](LICENSE-APACHE)

A licença do código é **MIT OU Apache-2.0**, mas algumas imagens, marcas, fontes e outros recursos podem ter regras próprias. Os arquivos originais `docs/brand/` contêm marca registrada da equipe ArtCraft; o aviso de licença exige que forks e versões modificadas removam essa identidade visual. A revisão de branding é obrigatória antes de redistribuir uma versão WMK.

**Personalização WMK:** Walney Moreira Klein — walneyk@hotmail.com

**Base:** [CADCraft / ArtCraft](https://github.com/storytold/cadcraft). Este fork não é afiliado à Autodesk.
