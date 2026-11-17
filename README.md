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

## Compilar no Windows

Requer Rust recente, com toolchain MSVC e dependências de compilação compatíveis. O README original descreve Rust 1.95+ para desktop/web.

```powershell
git clone https://github.com/WalneyNF/WMK-CAD-Studio.git
cd WMK-CAD-Studio
cargo run --release -p cadcraft -- --sample
```

O pacote e o executável da base ainda usam o nome técnico **`cadcraft`**. A troca do nome interno, ícones e demais recursos de marca WMK precisa ser feita com cuidado, sem quebrar os builds.

### Compilar pelo GitHub Actions

O fork traz um workflow original de release em `.github/workflows/release.yml` que utiliza ações externas da equipe original e parâmetros de assinatura. Para um teste independente, este fork também oferece o workflow **WMK Windows Portable**, que tenta compilar um EXE sem instalador e publica o artefato do job se a compilação passar.

Veja [Actions](https://github.com/WalneyNF/WMK-CAD-Studio/actions). **Não afirmar build concluído sem consultar a execução.**

## Relação com os outros projetos WMK

| Projeto | Responsabilidade |
|---|---|
| [WMK CAD Studio](https://github.com/WalneyNF/WMK-CAD-Studio) | Desenho 2D preciso e DXF/DWG |
| [WMK BIM Studio](https://github.com/WalneyNF/WMK-BIM-STUDIO) | Modelos BIM: paredes, portas, janelas, pisos, telhados, IFC |
| [WMK Architecture Studio](https://github.com/WalneyNF/WMK-Architecture-Studio) | Ponte CAD → BIM, validação geométrica, futura integração OCR/IA |

**Primeira integração já preparada:** no WMK Architecture Studio existe um conversor experimental de linhas DXF, na camada `WMK_PAREDES`, para comandos `wall` do WMK BIM Studio. Ainda não existe sincronização automática ou reconhecimento completo de imagem.

## Identidade visual

Os logos WMK CAD Studio e WMK Architecture Studio foram aprovados. A aplicação das imagens binárias nos repositórios e a substituição completa dos ícones antigos são tarefas distintas da tradução deste README e devem ser verificadas no build. **Não distribuir os logos e nomes registrados da equipe ArtCraft como identidade do fork.**

## Documentação, autoria e licenças

- [README original em inglês](README.original.en.md)
- [Roadmap original](ROADMAP.md)
- [Documentação técnica](docs/)
- [Atribuições](ATTRIBUTION.md), [avisos legais](NOTICE), [MIT](LICENSE-MIT) e [Apache-2.0](LICENSE-APACHE)

A licença do código é **MIT OU Apache-2.0**, mas algumas imagens, marcas, fontes e outros recursos podem ter regras próprias. Os arquivos originais `docs/brand/` contêm marca registrada da equipe ArtCraft; o aviso de licença exige que forks e versões modificadas removam essa identidade visual. A revisão de branding é obrigatória antes de redistribuir uma versão WMK.

**Personalização WMK:** Walney Moreira Klein — walneyk@hotmail.com

**Base:** [CADCraft / ArtCraft](https://github.com/storytold/cadcraft). Este fork não é afiliado à Autodesk.
