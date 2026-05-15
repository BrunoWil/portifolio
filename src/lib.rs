use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[derive(Deserialize)]
struct Contato {
    email: String,
    linkedin: String,
    github: String,
}

#[derive(Deserialize)]
struct Projeto {
    categoria: String,
    titulo: String,
    descricao: String,
    tags: Vec<String>,
}

#[derive(Deserialize)]
struct PortfolioData {
    #[serde(rename = "nomeCurto")]
    nome_curto: String,

    #[serde(rename = "nomeCompleto")]
    nome_completo: String,

    resumo: String,
    contato: Contato,
    projetos: Vec<Projeto>,
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    spawn_local(async {
        if let Err(e) = carregar_portfolio().await {
            web_sys::console::error_1(&e);
        }
    });

    Ok(())
}

async fn carregar_portfolio() -> Result<(), JsValue> {
    let url = "https://raw.githubusercontent.com/BrunoWil/Portifolio/master/dados.json";

    let timestamp = js_sys::Date::now();
    let fetch_url = format!("{}?t={}", url, timestamp);

    let response = Request::get(&fetch_url)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let data: PortfolioData = response
        .json()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let window =
        window().ok_or_else(|| JsValue::from_str("Janela global não encontrada"))?;

    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("Documento não encontrado"))?;

    // CSS GLOBAL
    if let Some(head) = document.head() {
        let style = document.create_element("style")?;

        style.set_inner_html(r#"
            :root {
                --primary: #2563eb;
                --primary-dark: #1d4ed8;
                --bg-color: #f8fafc;
                --text-main: #0f172a;
                --text-muted: #64748b;
                --card-bg: #ffffff;
            }

            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
                font-family: 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
            }

            html {
                scroll-behavior: smooth;
            }

            body {
                background-color: var(--bg-color);
                color: var(--text-main);
                line-height: 1.6;
                min-height: 100vh;
                display: flex;
                flex-direction: column;
            }

            nav {
                background-color: rgba(255,255,255,0.95);
                backdrop-filter: blur(8px);
                box-shadow: 0 2px 10px rgba(0,0,0,0.05);

                position: sticky;
                top: 0;
                left: 0;
                width: 100%;

                z-index: 1000;

                display: flex;
                justify-content: space-between;
                align-items: center;

                padding: 1rem 5%;
            }

            .logo {
                font-size: 1.5rem;
                font-weight: 700;
                color: var(--primary);
                text-decoration: none;
                letter-spacing: 1px;
            }

            .nav-links {
                display: flex;
                gap: 1.5rem;
                align-items: center;
            }

            .nav-links a {
                text-decoration: none;
                color: var(--text-main);
                font-weight: 500;
                transition: color 0.3s;
            }

            .nav-links a:hover {
                color: var(--primary);
            }

            section {
                padding: 5rem 5%;
                max-width: 1200px;
                margin: 0 auto;
            }

            .section-title {
                text-align: center;
                font-size: 2.2rem;
                margin-bottom: 3rem;
                position: relative;
            }

            .section-title::after {
                content: '';
                display: block;
                width: 60px;
                height: 4px;
                background-color: var(--primary);
                margin: 10px auto 0;
                border-radius: 2px;
            }

            #inicio {
                min-height: 80vh;

                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;

                text-align: center;
            }

            #inicio h1 {
                font-size: 3.5rem;
                margin-bottom: 1rem;
            }

            #inicio h1 span {
                color: var(--primary);
            }

            #inicio p {
                font-size: 1.2rem;
                color: var(--text-muted);
                max-width: 600px;
                margin-bottom: 2rem;
            }

            .btn {
                background-color: var(--primary);
                color: white;
                padding: 0.8rem 2rem;
                border-radius: 8px;
                text-decoration: none;
                font-weight: 600;
                transition: 0.3s;
            }

            .btn:hover {
                background-color: var(--primary-dark);
                transform: translateY(-2px);
            }

            .projects-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                gap: 2rem;
            }

            .project-card {
                background-color: var(--card-bg);
                border-radius: 12px;
                overflow: hidden;

                box-shadow: 0 4px 6px rgba(0,0,0,0.05);
                transition: 0.3s;
            }

            .project-card:hover {
                transform: translateY(-5px);
                box-shadow: 0 10px 20px rgba(0,0,0,0.1);
            }

            .project-img {
                width: 100%;
                height: 200px;

                background-color: #cbd5e1;

                display: flex;
                align-items: center;
                justify-content: center;

                color: var(--text-muted);
                font-weight: 600;
            }

            .project-info {
                padding: 1.5rem;
            }

            .project-info h3 {
                margin-bottom: 0.5rem;
            }

            .tags {
                display: flex;
                gap: 0.5rem;
                margin-top: 1rem;
                flex-wrap: wrap;
            }

            .tag {
                background-color: #e2e8f0;
                color: #475569;

                padding: 0.3rem 0.8rem;
                border-radius: 20px;

                font-size: 0.8rem;
                font-weight: 600;
            }

            #contato {
                text-align: center;
                background-color: var(--card-bg);

                border-radius: 16px;
                padding: 4rem 2rem;

                box-shadow: 0 4px 20px rgba(0,0,0,0.03);
                margin-bottom: 3rem;
            }

            .contact-links {
                display: flex;
                justify-content: center;
                gap: 2rem;

                margin-top: 2rem;
                flex-wrap: wrap;
            }

            .contact-links a {
                color: var(--primary);
                text-decoration: none;
                font-weight: 600;
                font-size: 1.1rem;
            }

            footer {
                text-align: center;
                padding: 2rem;
                background-color: var(--text-main);
                color: white;
                margin-top: auto;
            }

            @media (max-width: 768px) {
                #inicio h1 {
                    font-size: 2.5rem;
                }

                .nav-links {
                    display: none;
                }
            }
        "#);

        head.append_child(&style)?;
    }

    // PROJETOS
    let mut projetos_html = String::new();

    for proj in data.projetos {
        let tags_html: String = proj
            .tags
            .iter()
            .map(|tag| format!(r#"<span class="tag">{}</span>"#, tag))
            .collect();

        let card = format!(
            r#"
            <div class="project-card">
                <div class="project-img">{}</div>

                <div class="project-info">
                    <h3>{}</h3>
                    <p>{}</p>

                    <div class="tags">
                        {}
                    </div>
                </div>
            </div>
            "#,
            proj.categoria,
            proj.titulo,
            proj.descricao,
            tags_html
        );

        projetos_html.push_str(&card);
    }

    let ano = js_sys::Date::new_0()
        .get_full_year()
        .to_string();

    // HTML
    let body_html = format!(
        r##"
        <nav>
            <a href="#inicio" class="logo nav-scroll">
                {}.
            </a>

            <div class="nav-links">
                <a href="#inicio" class="nav-scroll">Início</a>
                <a href="#projetos" class="nav-scroll">Projetos</a>
                <a href="#contato" class="nav-scroll">Contato</a>
            </div>
        </nav>

        <section id="inicio">
            <h1>
                Olá, eu sou <span>{}</span>
            </h1>

            <p>{}</p>

            <a href="#projetos" class="btn nav-scroll">
                Ver Meus Trabalhos
            </a>
        </section>

        <section id="projetos">
            <h2 class="section-title">
                Meus Projetos
            </h2>

            <div class="projects-grid">
                {}
            </div>
        </section>

        <section id="contato">
            <h2 class="section-title">
                Vamos Conversar?
            </h2>

            <p>
                Busco aprimoramento contínuo e estou sempre aberto a discutir novos desafios e projetos na área de software.
            </p>

            <div class="contact-links">
                <a href="mailto:{}">
                    📧 {}
                </a>

                <a href="{}" target="_blank">
                    💼 LinkedIn
                </a>

                <a href="{}" target="_blank">
                    💻 GitHub
                </a>
            </div>
        </section>

        <footer>
            <p>
                &copy; {} {}. Todos os direitos reservados.
            </p>
        </footer>
        "##,
        data.nome_completo,
        data.nome_curto,
        data.resumo,
        projetos_html,
        data.contato.email,
        data.contato.email,
        data.contato.linkedin,
        data.contato.github,
        ano,
        data.nome_completo
    );

    if let Some(body) = document.body() {
        body.set_inner_html(&body_html);
        body.set_attribute("class", "")?;
    }

    // SCROLL SUAVE CORRIGIDO
    let nav_links = document.query_selector_all(".nav-scroll")?;

    for i in 0..nav_links.length() {
        if let Some(node) = nav_links.get(i) {
            let element: web_sys::HtmlElement = node.dyn_into()?;

            let href = element.get_attribute("href").unwrap_or_default();

            let target_id = href.trim_start_matches('#').to_string();

            let document_clone = document.clone();

            let closure = Closure::<dyn FnMut(_)>::new(
                move |event: web_sys::MouseEvent| {
                    event.prevent_default();

                    if let Some(target) =
                        document_clone.get_element_by_id(&target_id)
                    {
                        let rect = target.get_bounding_client_rect();

                        let window = web_sys::window().unwrap();

                        let scroll_y =
                            window.scroll_y().unwrap_or(0.0);

                        let offset = 70.0;

                        let target_position =
                            rect.top() + scroll_y - offset;

                        let options =
                            web_sys::ScrollToOptions::new();

                        options.set_top(target_position);

                        options.set_behavior(
                            web_sys::ScrollBehavior::Smooth
                        );

                        window.scroll_to_with_scroll_to_options(
                            &options
                        );
                    }
                },
            );

            element.add_event_listener_with_callback(
                "click",
                closure.as_ref().unchecked_ref(),
            )?;

            closure.forget();
        }
    }

    Ok(())
}