use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

const REPO: &str = "ANANDHUAJITH/ketchup";
const ASCII_SRC: &str = include_str!("../assets/ketchup-compact.txt");

/// One colored run of the ascii-art banner, ready to drop straight into a <span style="color:...">.
struct AsciiRun {
    color: &'static str,
    text: String,
}

/// Parse the fastfetch-style `colors N N N` + `${cN}` ascii file into colored runs.
/// The three terminal-256 color indices in the header map to Ketchup's brand palette
/// (same mapping used in `etc/skel/.config/fastfetch/config.jsonc` on the OS itself),
/// so the web banner and the terminal banner read as the same logo.
fn parse_ascii(src: &str) -> Vec<AsciiRun> {
    let palette: [&str; 5] = ["#e63a2e", "#f3e6dc", "#3a0f10", "#e63a2e", "#f3e6dc"];
    let mut lines = src.lines();
    let _header = lines.next(); // "colors 160 233 255" - indices only matter to the terminal renderer
    let body: String = lines.collect::<Vec<_>>().join("\n");

    let mut runs = Vec::new();
    let mut color = palette[0];
    let mut buf = String::new();
    let bytes = body.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if body[i..].starts_with("${c") {
            if let Some(end) = body[i..].find('}') {
                let tag = &body[i..i + end + 1]; // e.g. "${c1}"
                if let Some(n) = tag[3..tag.len() - 1].parse::<usize>().ok() {
                    if !buf.is_empty() {
                        runs.push(AsciiRun { color, text: std::mem::take(&mut buf) });
                    }
                    color = palette.get(n.saturating_sub(1)).copied().unwrap_or(palette[0]);
                }
                i += end + 1;
                continue;
            }
        }
        let ch = body[i..].chars().next().unwrap();
        buf.push(ch);
        i += ch.len_utf8();
    }
    if !buf.is_empty() {
        runs.push(AsciiRun { color, text: buf });
    }
    runs
}

#[function_component(Banner)]
fn banner() -> Html {
    let runs = parse_ascii(ASCII_SRC);
    html! {
        <pre class="ascii-banner" aria-hidden="true">
            { for runs.into_iter().map(|r| html! { <span style={format!("color:{}", r.color)}>{ r.text }</span> }) }
        </pre>
    }
}

#[derive(PartialEq, Clone)]
struct Feature {
    title: &'static str,
    body: &'static str,
}

const FEATURES: &[Feature] = &[
    Feature { title: "Limine, everywhere", body: "A single bootloader across BIOS, UEFI and hybrid USB, with hash-pinned kernel/initramfs and automatic GRUB fallback when a disk's /boot can't be read by Limine." },
    Feature { title: "Rust-first userland", body: "alacritty, eza, bat, fd, ripgrep, sd, dust, procs, bottom, zoxide, starship, yazi, helix, zellij and more ship by default, alongside KDE Plasma and linux-zen." },
    Feature { title: "Ketchup Lens", body: "Trace a real running program, or step through a toy CPU/MMU/cache/RAM/disk one instruction at a time - a teaching tool built into the OS itself." },
    Feature { title: "Tuned, not just themed", body: "zram with zstd, BBR, ananicy-cpp, scx_lavd, systemd-oomd and weekly fstrim are on by default; the live initramfs itself uses zstd so the boot-to-splash gap stays short." },
    Feature { title: "Built-in self-checks", body: "ketchup-audit, ketchup-doctor and ketchup-irqfix turn \"something feels off\" into a concrete diagnosis instead of a forum post." },
    Feature { title: "Offline-capable installer", body: "Calamares installs straight from the live image - no network required - onto btrfs + zstd + subvolumes by default." },
];

const INSTALL_CMD: &str =
    "curl -fsSL https://api.github.com/repos/ANANDHUAJITH/ketchup/releases/latest | grep -o 'https://[^\"]*\\.iso' | head -n1";

#[function_component(App)]
fn app() -> Html {
    let copied = use_state(|| false);

    let on_copy = {
        let copied = copied.clone();
        Callback::from(move |_| {
            let copied = copied.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(win) = web_sys::window() {
                    let clipboard = win.navigator().clipboard();
                    let promise = clipboard.write_text(INSTALL_CMD);
                    let _ = JsFuture::from(promise).await;
                    copied.set(true);
                }
            });
        })
    };

    html! {
        <>
            <header class="hero">
                <Banner />
                <h1>{ "Ketchup" }</h1>
                <p class="tagline">{ "A fast, slim, Arch-based desktop OS: KDE Plasma + linux-zen + Rust tooling + Distrobox." }</p>
                <div class="cta-row">
                    <a class="btn primary" href={format!("https://github.com/{REPO}/releases")}>{ "Download the ISO" }</a>
                    <a class="btn" href={format!("https://github.com/{REPO}")}>{ "Source on GitHub" }</a>
                </div>
                <div class="install-cmd" onclick={on_copy} title="click to copy">
                    <code>{ INSTALL_CMD }</code>
                    <span class="hint">{ if *copied { "copied - paste into a terminal or download manager" } else { "click to copy: finds the latest ISO's direct URL" } }</span>
                </div>
            </header>

            <main>
                <section class="features">
                    { for FEATURES.iter().map(|f| html! {
                        <article class="feature">
                            <h2>{ f.title }</h2>
                            <p>{ f.body }</p>
                        </article>
                    }) }
                </section>

                <section class="about">
                    <h2>{ "Built from CachyOS, rebranded and re-tuned" }</h2>
                    <p>
                        { "Ketchup starts from the " }
                        <a href="https://github.com/CachyOS/CachyOS-Live-ISO">{ "CachyOS Live ISO" }</a>
                        { " profile and rebuilds the boot path, branding, installer, and a chunk of the default \
                           tool set on top of it. The ISO itself is built by GitHub Actions from the repo below - \
                           nothing here hides what the OS actually does." }
                    </p>
                </section>
            </main>

            <footer>
                <nav>
                    <a href={format!("https://github.com/{REPO}")}>{ "Ketchup source" }</a>
                    <a href={format!("https://github.com/{REPO}/discussions")}>{ "Discussions" }</a>
                    <a href={format!("https://github.com/{REPO}/issues")}>{ "Issues" }</a>
                    <a href={format!("https://github.com/{REPO}#readme")}>{ "Documentation" }</a>
                </nav>
                <p class="small">{ "This site is itself written in Rust, compiled to WebAssembly with Yew + Trunk, and deployed to GitHub Pages." }</p>
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
