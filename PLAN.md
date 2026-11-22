# Plan: Mr. Support — Soporte remoto con IA sobre RustDesk (código abierto, Rust)

*Preparado para Miguel — 2026-11-21. Nombre elegido: **Mr. Support** (tras descartar Ojo por feedback familiar: su esposa sintió que Ojo suena a espionaje y propuso Mitre Support; Miguel eligió Mr. Support). Logo idea de su hija Emma: lentes con bigote tipo Camilo o estilo Pringles. Decisión: enjambre multi-agente (swarm) desde el inicio, todo o nada. Investigación de RustDesk desde el repo oficial; lo no verificado está marcado como pendiente. No se publicó ni envió nada.*

## 1. Visión y alcance del MVP

**Visión:** **Mr. Support** — una herramienta de escritorio remoto de código abierto, construida sobre RustDesk, en la que un enjambre de agentes IA (p. ej. Muse) puede ver la pantalla del equipo del cliente y ejecutar acciones (clic, tecleo, navegación) en paralelo para dar soporte técnico completo, siempre con el usuario presente, viendo todo y pudiendo revocar en cualquier momento.

**Tagline:** "Mr. Support — AI support, human in control."

**Descripción corta:** Mr. Support is an open-source, Rust-based remote desktop for AI-assisted technical support. A swarm of specialized AI agents can see the screen and act in parallel to fix problems, while the user supervises, approves each action, and can revoke access instantly.

**Logo (idea de Emma, hija de Miguel):** cara amigable con lentes y bigote, estilo Camilo (cantante) o tipo Pringles (Mr. Pringles). Lentes = ve la pantalla, bigote = personaje confiable y simpático, no espía. Paleta: por definir, mantener amigable y profesional.

**Caso de uso inicial (MVP):** soporte remoto con IA para los clientes de Miguel. Ejemplo concreto: dar soporte a la Mac de una doctora (instalar/configurar software, resolver problemas de apps, guiar tareas) sin que Miguel tenga que estar en AnyDesk + terminal al mismo tiempo. La IA ve la pantalla, propone/ejecuta acciones, y el cliente aprueba y supervisa.

**Requisito multi-plataforma (decisión 2026-11-21):** debe funcionar en **Windows, macOS, Linux, Android e iOS**, como AnyDesk y TeamViewer. No es solo macOS: el diseño de captura/input y la UI deben ser cross-platform desde el inicio (RustDesk ya es cross-platform; la capa de agente debe abstraer por SO).

**Alcance del MVP:**
- **Multi-plataforma desde el inicio: Windows, macOS, Linux, Android e iOS** (como AnyDesk/TeamViewer). Priorizar macOS para piloto con clientes actuales, pero la arquitectura debe ser cross-platform (captura/input abstraídos por SO).
- Una sesión a la vez, con aprobación explícita del cliente por sesión.
- **Enjambre multi-agente desde el inicio (decisión 2026-11-21, todo o nada):** coordinador + agentes especializados en paralelo (diagnóstico, red, apps, seguridad, archivos). El coordinador serializa acciones de input (cola única hacia enigo) para evitar conflictos.
- Acciones básicas: clic, doble clic, tecleo, atajos de teclado, scroll, captura de pantalla para la IA.
- Modo "aprobar cada acción" al inicio; autonomía acotada después.
- Audit log completo de acciones (con agente que ejecutó cada una).
- Relay/ID server autoalojado (hbbs/hbbr) para que los datos no pasen por terceros.
- Fuera de alcance del MVP: acceso desatendido, multi-sesión, marketplace de agentes.

## 2. RustDesk

### Licencia (verificada)

- El repositorio oficial declara **AGPL-3.0** (GNU Affero General Public License v3.0): así lo muestra la ficha del repo en GitHub ("License: GNU Affero General Public License v3.0 (AGPL-3.0)") [1]. El servidor OSS (rustdesk-server) también es AGPL-3.0 [2]. Wikipedia lo registra como AGPL-3.0-or-later [3].
- **Nota de verificación:** no pude descargar el archivo LICENCE directamente (falló la petición), pero la metadata del repo de GitHub (que se deriva de ese archivo) y tres fuentes independientes coinciden en AGPL-3.0. *Pendiente:* confirmar el encabezado exacto ("AGPL-3.0" vs "AGPL-3.0-or-later") abriendo el archivo en el repo.
- **Qué implica AGPL-3.0 para un derivado:**
  - Copyleft fuerte: cualquier obra derivada (fork) debe distribuirse bajo la misma licencia AGPL-3.0, con código fuente disponible.
  - Cláusula de red (sección 13): si corres una versión modificada como servicio en red (p. ej. un SaaS de soporte), debes ofrecer el código fuente completo a los usuarios que interactúan con él por la red. Esto es clave para el modelo de negocio de Miguel si algún día ofrece el servicio hospedado.
  - No puedes relicenciar el código de RustDesk bajo MIT/Apache; solo puedes añadir código nuevo bajo otra licencia si es separable (ver sección 5).
  - Marca: "RustDesk" es marca de Purslane Tech Pte. Ltd.; un fork no puede usar ese nombre [4]. Hay que renombrar el proyecto.

### Arquitectura general (verificada desde README y docs)

- Un solo binario con dos roles según argumentos CLI: **Controller** (quien inicia la sesión y renderiza) y **Controlled Host** (quien captura y procesa input) [5].
- Cajas principales (README, "File Structure") [1]:
  - `libs/scrap`: captura de pantalla. Usa APIs nativas por plataforma: ScreenCaptureKit en macOS, DXGI en Windows, PipeWire/X11 en Linux [6].
  - `libs/enigo`: control de teclado/mouse específico por plataforma (inyección de input) [1].
  - `libs/clipboard`: portapapeles entre plataformas [1].
  - `libs/hbb_common`: códec de video, configuración, wrapper TCP/UDP y utilidades compartidas con el servidor; es un submódulo git compartido con rustdesk-server [1][7].
  - `libs/base`: protobuf, transferencia de archivos, código de plataforma solo-cliente [1].
  - `src/server/`: servicios de audio/clipboard/input/video y conexiones de red, organizados con un framework de servicios (`ServiceTmpl`) donde cada servicio corre como tarea async independiente coordinada por un connection manager [5][6].
  - `src/client.rs`: inicia la conexión peer [1].
  - `src/rendezvous_mediator.rs`: se comunica con el servidor rendezvous (hbbs), espera conexión directa (TCP hole punching) o relay [1][5].
  - `flutter/`: UI actual (Flutter, escritorio y móvil); la UI Sciter está deprecada [1].
- Transporte: protocolo propio sobre TCP, KCP o WebRTC; mensajes protobuf; Tokio para I/O async; Rust 1.75+ [6][5].
- Video: libvpx (VP8/VP9), aom (AV1), libyuv; audio con opus [1][6].
- Servidor (rustdesk-server, también AGPL-3.0) [2]:
  - `hbbs`: servidor ID/rendezvous (puerto 21116 por defecto).
  - `hbbr`: servidor relay (puerto 21117).
  - `rustdesk-utils`: utilidades CLI.
  - Se puede usar el servidor público de RustDesk, autoalojar el propio, o escribir uno propio (rustdesk-server-demo es el punto de partida más simple) [1][2].
- Conexión: P2P directo vía TCP hole punching cuando es posible; si NAT lo bloquea, fallback a relay hbbr con tráfico cifrado [6][5].
- Seguridad base: cifrado extremo a extremo (E2EE) en la sesión; la clave pública del servidor se configura en los clientes [6][2].

### Puntos de extensión para la capa de agente IA

1. **Tap de frames en `libs/scrap` / servicio de video (`src/server/`)**: los frames crudos existen antes de codificar. Se puede añadir un "tap" que copie frames (o frames reducidos) hacia un canal lateral para la IA, sin tocar el stream de video humano. Es el punto más limpio: no hay que decodificar el stream codificado.
2. **Servicio de input (`src/server/input_service.rs` + `libs/enigo`)**: las acciones de la IA (clic en x,y, tecleo) se traducen a llamadas a enigo, igual que hoy se traducen los eventos del humano. Aquí se inserta la política de aprobación: cada acción de la IA pasa por un validador antes de ejecutarse.
3. **Protocolo protobuf (`libs/base` / `hbb_common`)**: añadir tipos de mensaje para control de agente (p. ej. `AgentActionRequest`, `AgentActionResult`, `ConsentRequest`). Ojo: `hbb_common` es submódulo compartido con el servidor; cambios ahí cuestan un round-trip. El código solo-cliente va en `libs/base` (miembro normal del workspace) [7].
4. **UI Flutter (`flutter/`)**: overlay de consentimiento, indicador "IA operando", botón de pánico/revocación, visor de acciones. El bridge Rust↔Flutter es flutter_rust_bridge [6].
5. **Rendezvous/relay**: se puede autoalojar hbbs/hbbr sin tocar código; para sesiones de agente, considerar un "modo agente" en la negociación (capability negotiation) para que el host sepa que la sesión es de IA y aplique políticas distintas.
6. **Alternativa sin fork profundo**: proceso sidecar separado que se comunica por IPC/sockets con el host y usa el protocolo como cliente. Menos invasivo, pero más limitado (no accede al tap de frames interno sin cambios en RustDesk).

## 3. Capa de agente IA

### Cómo ve la pantalla la IA

- **Tap de frames**: el host expone un canal de frames crudos (desde `scrap`) hacia el agente. Para la IA no hace falta 60 fps: 1–5 fps a resolución reducida (p. ej. 1280×720 o escala 0.5) basta para un loop de decisión, y reduce costo de tokens/ancho de banda. El stream de video humano sigue en alta calidad por separado.
- **Loop de computer-use**: captura → modelo de visión decide → acción → nueva captura para verificar. Cada acción se valida contra la captura posterior (¿el clic cayó donde debía? ¿apareció lo esperado?).
- **Grounding (coordenadas)**: el modelo debe devolver coordenadas de clic/teclas en el espacio de la pantalla. Opciones: modelos "computer use" comerciales (p. ej. Claude Computer Use, OpenAI CUA) vía API, o modelos abiertos con grounding (p. ej. familia UI-TARS, CogAgent) autoalojados. Recomendación: diseñar la capa de agente agnóstica al modelo (interfaz `Agent::next_action(screenshot, history) -> Action`), para poder cambiar de proveedor o pasar a local.
- **Contexto**: historial de acciones + capturas recientes; opcionalmente OCR/elementos de accesibilidad (AX en macOS) para anclar clics a elementos en vez de píxeles puros — más robusto que coordenadas ciegas.

### Enjambre de agentes (multi-agente) — idea de Miguel (2026-11-21)

La idea: en vez de un solo agente, un **enjambre de agentes especializados** entra al soporte en paralelo para resolver más rápido. Ejemplos de especialidades: diagnóstico del sistema, red/conectividad, aplicaciones, seguridad, archivos. Un agente coordinador reparte el trabajo, consolida resultados y presenta un solo plan al humano (Miguel/cliente) para aprobación.

- **Arquitectura:** coordinador + N agentes especializados, cada uno con su loop de visión/acción acotado a su dominio (p. ej. el agente de red solo puede tocar ajustes de red). Comunicación entre agentes por mensajes estructurados; memoria compartida de sesión (qué vio cada uno, qué hizo).
- **Ventajas:** velocidad (trabajo en paralelo), especialización (cada agente con prompts/herramientas de su dominio), resiliencia (si uno falla, otros siguen).
- **Riesgos nuevos:** conflictos de acciones (dos agentes tocando lo mismo a la vez) → se resuelve con el coordinador serializando las acciones de input (una sola cola de input hacia enigo) y lock por recurso; costo de tokens mayor (N agentes viendo pantalla) → compartir capturas entre agentes en vez de que cada uno pida la suya; complejidad de depuración.
- **Supervisión humana:** el enjambre nunca actúa sin el modelo de consentimiento de la sección 4; el coordinador presenta el plan consolidado y el humano aprueba. Cada acción queda en el audit log con el agente que la ejecutó.
- **Alcance:** **DESDE EL INICIO (decisión 2026-11-21, todo o nada).** El MVP incluye el enjambre multi-agente: coordinador + agentes especializados en paralelo. El coordinador serializa las acciones de input (cola única hacia enigo) y presenta el plan consolidado para aprobación humana. Requiere definir el protocolo inter-agente y la cola única de input como parte del MVP.

### APIs del SO por plataforma

- **macOS** (prioridad MVP):
  - Captura: ScreenCaptureKit (moderno, macOS 12.3+) o CGDisplayStream (legado). Requiere permiso **Screen Recording** (TCC).
  - Input: CGEvent (CGEventCreateMouseEvent / teclado) vía enigo; requiere permiso **Accessibility**. Sin Accessibility, la inyección de clics/teclas no funciona.
  - Implicación UX: el primer arranque debe guiar al usuario a otorgar ambos permisos en Ajustes del sistema; notarización de la app para distribución.
  - *(Verificar en código: qué API usa exactamente `scrap`/`enigo` en macOS hoy — ScreenCaptureKit está documentado en fuentes secundarias [6]; los permisos TCC son requisito del SO, conocimiento general.)*
- **Windows**:
  - Captura: DXGI Desktop Duplication [6].
  - Input: SendInput vía enigo. Escritorio seguro (UAC) requiere servicio elevado; considerar modo "asistencia" con el usuario presente para evitar el problema del secure desktop.
- **Linux**:
  - Captura: X11 (XShm/XFixes) o PipeWire portal en Wayland [6].
  - Input: XTest vía libxdo (enigo) en X11; en Wayland, uinput/portal — más restrictivo. Wayland es el mayor riesgo de compatibilidad.

## 4. Modelo de seguridad y consentimiento

Diseñado para el caso sensible: equipo de una doctora con datos de salud (datos personales sensibles).

- **Aprobación de sesión**: ninguna sesión de IA sin consentimiento explícito del usuario en ese momento (diálogo con quién/qué/para qué). Sin acceso desatendido en el MVP. Opcional: "recordar para esta tarea" con expiración.
- **Visibilidad total**: indicador persistente "IA operando" (icono + borde en pantalla), panel que muestra qué ve la IA (miniatura en vivo) y qué acción está por ejecutar antes de ejecutarla (en modo supervisado).
- **Revocación**: botón de pánico visible + atajo de teclado; cualquier input del usuario real preempta/pausa a la IA al instante; la sesión termina al cerrar el diálogo o al expirar un timeout de inactividad.
- **Audit log**: registro append-only con timestamp, acción (clic en x,y / texto tecleado / app enfocada), captura antes/después, resultado, y quién aprobó. Exportable (JSON) para el cliente. Idealmente firmado/hasheado por evento para integridad.
- **Privacidad**:
  - Minimización: enviar a la IA solo los frames necesarios, a baja resolución/fps; opción de inferencia local (modelo en el equipo) para que los datos de salud nunca salgan de la máquina.
  - Redacción: opción de tapar regiones (p. ej. ventana del expediente) antes de enviar frames a la nube.
  - Sin grabación de sesión por defecto; si se graba, con consentimiento explícito y cifrado.
  - Transporte E2EE (ya lo da RustDesk) + relay autoalojado (hbbs/hbbr propio) para que el video no pase por infraestructura de terceros.
  - Marco legal: en México, los datos de salud son datos personales sensibles (LFPDPPP) → consentimiento expreso y por escrito; en EE. UU. aplicaría HIPAA si hay clientes allá. El diseño (consentimiento por sesión + audit log + minimización) apunta a cumplir ambos.
- **Anti prompt-injection**: el contenido en pantalla (texto malicioso en una página) no debe poder ordenar a la IA acciones fuera de política. Mitigación: la IA solo recibe instrucciones del operador (Miguel/la sesión aprobada), las acciones pasan por allow-list de políticas, y acciones destructivas (borrar, enviar, comprar, cambiar contraseñas) siempre requieren aprobación humana.
- **Superficie de ataque**: la IA con control de input es un objetivo de alto valor → autenticación fuerte de sesión (código de un solo uso + confirmación local), canal cifrado, y principio de mínimo privilegio (la IA opera como el usuario, nunca como root/admin salvo aprobación explícita por acción).

## 5. Recomendación de licencia para el proyecto nuevo

- **Si se hace fork de RustDesk (se modifica su código): la licencia resultante debe ser AGPL-3.0.** No hay opción legal de relicenciar el código derivado bajo MIT/Apache. Además, la cláusula de red (sección 13) obliga a ofrecer el código fuente si se opera como servicio en red [verificado en fuentes sobre AGPL, sección 2].
- **Recomendación práctica (híbrida):**
  - **Núcleo derivado de RustDesk → AGPL-3.0** (obligatorio). Esto además cumple el objetivo de Miguel: código abierto que devuelve a la comunidad, y la AGPL impide que un tercero tome el trabajo y lo cierre.
  - **Capa de agente como proceso separado (sidecar) que se comunica por protocolo/IPC a distancia de brazo ("mere aggregation") → Apache-2.0.** Justificación: el sidecar es obra separada que no deriva del código de RustDesk; Apache-2.0 da grant de patentes (mejor para adopción por empresas) y permite uso comercial sin copyleft, lo que facilita que otros integren el agente. Riesgo: la frontera "obra derivada vs. agregación" es legal y discutible; si el sidecar linkea código de RustDesk (p. ej. importa crates internos), se contamina a AGPL. Mitigación: comunicar solo por sockets/protocolo documentado, sin importar crates internos.
  - **Alternativa simple (recomendada si se quiere cero fricción legal): todo AGPL-3.0.** Máxima coherencia con la filosofía de Miguel ("aportar algo a la comunidad") y cero riesgo de incompatibilidad. La desventaja: menor adopción comercial.
- **Marca**: nombre elegido **Mr. Support** (no usar "RustDesk") [4]. Logo: lentes con bigote estilo Camilo/Pringles (idea de Emma).
- **Pendiente**: revisión legal real antes de publicar (esto es orientación, no asesoría legal). Verificar también la política de contribuciones de RustDesk si se piensa aportar upstream.

## 6. Hitos del MVP (en orden)

1. **M0 — Spike y validación (1–2 semanas).** Compilar RustDesk desde fuente, levantar hbbs/hbbr autoalojado (Docker), conectar dos equipos, verificar captura e input en macOS. Entregable: build funcionando + notas de setup.
2. **M1 — Tap de frames.** Instrumentar el pipeline de captura para exponer frames crudos (o reducidos) por un canal lateral (IPC/socket local) sin afectar el stream humano. Entregable: módulo/crate de tap + visor de prueba que muestra los frames.
3. **M2 — Loop de agente mínimo (macOS, HITL).** Screenshot → modelo de visión → una acción (clic/tecleo vía enigo) → verificación con nueva captura. Cada acción requiere aprobación humana. Entregable: loop funcional en una Mac, con modo "aprobar cada acción".
   **M2b — Enjambre multi-agente (desde el inicio, todo o nada).** Coordinador + agentes especializados en paralelo (diagnóstico, red, apps, seguridad, archivos). Protocolo inter-agente, memoria compartida de sesión, cola única de input serializada por el coordinador, plan consolidado para aprobación humana. Entregable: enjambre funcional con al menos 3 especialistas + coordinador en macOS.
4. **M3 — Consentimiento y seguridad MVP.** Diálogo de aprobación de sesión, indicador "IA operando", botón/atajo de revocación, audit log (acciones + capturas) exportable, E2EE vía canal RustDesk existente. Entregable: UX de consentimiento + log de auditoría.
5. **M4 — Autonomía acotada.** Políticas: allow-list de apps/acciones, niveles de riesgo (acciones destructivas siempre con aprobación), preemption por input del usuario, overlay que marca el objetivo del clic. Entregable: motor de políticas.
6. **M5 — Empaquetado macOS.** Instalador firmado/notarizado, guía de permisos (Screen Recording + Accessibility), auto-update opcional. Entregable: .dmg instalable.
7. **M6 — Piloto real.** Sesión supervisada con un cliente (p. ej. la Mac de la doctora) para una tarea acotada (instalar/configurar una app). Entregable: reporte del piloto + ajustes.
8. **M7 — Liberación pública.** Repo público **Mr. Support**, LICENSE(s), README, guía de contribución, demo en video. Entregable: v0.1 pública.

*(Fase 2, fuera del MVP: acceso desatendido con consentimiento previo, inferencia local, multi-sesión.)*

## 7. Riesgos y preguntas abiertas

**Riesgos:**
- **Confiabilidad del grounding**: los modelos de visión se equivocan de clic; se mitiga con verificación post-acción, reintentos y políticas conservadoras, pero nunca será 100%. Pregunta abierta: ¿qué tasa de error es aceptable para soporte a clientes?
- **Permisos de macOS**: Screen Recording + Accessibility son fricción de UX y requisito duro; la notarización y la distribución fuera de la App Store añaden trabajo. Pendiente: verificar el flujo exacto que usa RustDesk hoy.
- **AGPL y modelo de servicio**: si Miguel opera el servicio hospedado con código modificado, debe ofrecer el código fuente a los usuarios (sección 13). Decidir temprano si el modelo es "software para que cada cliente lo corra" vs. "servicio operado por Miguel".
- **Funciones Pro de RustDesk**: consola web, SSO, audit logs avanzados y gestión centralizada están en el Server Pro cerrado [4][6]; no se pueden tomar de ahí — habría que construirlos (el audit log del MVP es propio).
- **Prompt injection vía pantalla**: texto malicioso visible puede intentar manipular al agente. Mitigación con allow-lists y aprobación humana para acciones sensibles; sigue siendo riesgo residual.
- **Privacidad/regulatorio**: datos de salud (LFPDPPP en México; HIPAA en EE. UU.). La inferencia en la nube envía capturas de pantalla fuera del equipo → considerar inferencia local o redacción como requisito para clientes de salud.
- **Costo/latencia de la IA**: frames a modelo en la nube = tokens y ancho de banda por sesión; modelos locales = menos precisión. Decidir por caso.
- **Mantenimiento del fork**: RustDesk es grande (Rust + Flutter, submódulo hbb_common compartido con el servidor); seguir upstream cuesta. Pregunta abierta: ¿fork completo o sidecar con cambios mínimos upstream?
- **Wayland (Linux)**: captura/input muy restringidos; posponer Linux a fase 2.
- **Marca y comunidad**: renombrar el proyecto; decidir si se busca contribuir upstream (mejora la relación con la comunidad y reduce el costo de mantener el fork).

**Preguntas abiertas para Miguel:**
1. ¿Fork de RustDesk o sidecar externo con cambios mínimos? (Define licencia y esfuerzo.)
2. ¿Modelo de IA: API en la nube (Claude Computer Use u otro) o modelo local? (Define privacidad y costo.)
3. ¿El servicio lo opera Miguel (SaaS → AGPL sección 13 aplica de lleno) o cada cliente instala su instancia?
4. ~~¿Nombre del proyecto?~~ → **Resuelto 2026-11-21: Mr. Support.** Tagline: "Mr. Support — AI support, human in control." Logo idea de Emma: lentes con bigote tipo Camilo/Pringles.
5. ~~¿macOS primero confirmado, o también Windows desde el inicio?~~ → **Resuelto 2026-11-21: multi-plataforma desde el inicio — Windows, macOS, Linux, Android e iOS (como AnyDesk/TeamViewer).**
6. Enjambre multi-agente (desde el inicio): ¿coordinador central o peer-to-peer, y cómo se serializan las acciones de input para evitar conflictos entre agentes?

## Fuentes y estado de verificación

**Verificado desde el repo oficial / fuentes citadas:**
- Licencia AGPL-3.0 del cliente y del servidor OSS [1][2]; Wikipedia: AGPL-3.0-or-later [3].
- Estructura de archivos y componentes (scrap, enigo, hbb_common, base, server, rendezvous_mediator, flutter) [1][7].
- Roles Controller/Controlled Host en un solo binario; servicios async con ServiceTmpl [5].
- Transporte TCP/KCP/WebRTC, protobuf, Tokio, Rust 1.75+; captura nativa por plataforma (ScreenCaptureKit/DXGI/PipeWire/X11); códecs libvpx/aom/libyuv/opus; Flutter vía flutter_rust_bridge [6][5].
- Servidor: hbbs (ID/rendezvous, 21116) y hbbr (relay, 21117); autoalojable o servidor propio; rustdesk-server-demo como punto de partida [2][1].
- RustDesk Server Pro (cerrado) con consola web, SSO, audit logs, etc. [4][6].
- Marca RustDesk de Purslane Tech Pte. Ltd. [4].

**Pendiente de verificar (marcado como tal en el texto):**
- Encabezado exacto del archivo LICENCE (AGPL-3.0 vs AGPL-3.0-or-later): la descarga directa del archivo falló; la metadata del repo de GitHub confirma AGPL-3.0.
- API exacta de captura/input que usa `scrap`/`enigo` en cada plataforma en el código actual (las APIs por plataforma vienen de fuentes secundarias [6]).
- Flujo de permisos TCC (Screen Recording/Accessibility) tal como lo implementa RustDesk hoy.
- Detalles de la cláusula de red AGPL sección 13 aplicados al modelo de servicio concreto (requiere revisión legal).

[1] https://github.com/rustdesk/rustdesk
[2] https://github.com/rustdesk/rustdesk-server
[3] https://en.wikipedia.org/wiki/RustDesk
[4] https://marketplace.visualstudio.com/items?itemName=eren-tools.rustdesk-remote-access (nota de marca y licencia)
[5] https://deepwiki.com/rustdesk/rustdesk/2-architecture-overview
[6] https://openapps.pro/apps/rustdesk
[7] https://git.pyzyn.ru/mirrors/rustdesk/src/commit/350966b331bb873650668549b3fa4a42e81e6a81/AGENTS.md
