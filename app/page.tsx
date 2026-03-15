"use client";

import { useEffect, useRef, useState } from "react";

function Starfield() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    let raf: number;
    const resize = () => { canvas.width = window.innerWidth; canvas.height = window.innerHeight; };
    resize();
    window.addEventListener("resize", resize);
    const stars = Array.from({ length: 220 }, () => ({
      x: Math.random() * window.innerWidth,
      y: Math.random() * window.innerHeight,
      r: Math.random() * 1.6 + 0.3,
      speed: Math.random() * 0.1 + 0.02,
      opacity: Math.random(),
      td: Math.random() * 0.012 + 0.003,
      tdir: Math.random() > 0.5 ? 1 : -1,
    }));
    const draw = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      stars.forEach((s) => {
        s.opacity += s.td * s.tdir;
        if (s.opacity > 1 || s.opacity < 0.05) s.tdir *= -1;
        s.y += s.speed;
        if (s.y > canvas.height) { s.y = 0; s.x = Math.random() * canvas.width; }
        ctx.beginPath();
        ctx.arc(s.x, s.y, s.r, 0, Math.PI * 2);
        ctx.fillStyle = `rgba(255,255,255,${s.opacity})`;
        ctx.fill();
      });
      raf = requestAnimationFrame(draw);
    };
    draw();
    return () => { cancelAnimationFrame(raf); window.removeEventListener("resize", resize); };
  }, []);
  return (
    <canvas ref={canvasRef} style={{ position: "fixed", top: 0, left: 0, width: "100%", height: "100%", pointerEvents: "none", zIndex: 0 }} />
  );
}

export default function Home() {
  const [email, setEmail] = useState("");
  const [submitted, setSubmitted] = useState(false);

  return (
    <>
      {/* Google Fonts — Orbitron (sci-fi / alien movie font) */}
      <style>{`
        @import url('https://fonts.googleapis.com/css2?family=Orbitron:wght@400;600;700;800;900&display=swap');

        @keyframes blink {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.1; }
        }
        @keyframes glow-pulse {
          0%, 100% { filter: drop-shadow(0 0 18px rgba(255,255,255,0.25)); }
          50%       { filter: drop-shadow(0 0 42px rgba(255,255,255,0.55)); }
        }

        * { box-sizing: border-box; margin: 0; padding: 0; }
        body { background: #000; }
        ::selection { background: #fff; color: #000; }
        ::-webkit-scrollbar { width: 3px; }
        ::-webkit-scrollbar-track { background: #000; }
        ::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.15); }
        input::placeholder { color: rgba(255,255,255,0.25); font-family: 'Orbitron', monospace; }

        .orbitron { font-family: 'Orbitron', 'Courier New', monospace; }

        nav a.github-btn:hover {
          border-color: rgba(255,255,255,0.7) !important;
          color: #fff !important;
        }
        footer a:hover { color: rgba(255,255,255,0.6) !important; }
        button.notify-btn:hover { background: rgba(255,255,255,0.85) !important; }
      `}</style>

      <div className="orbitron" style={{ minHeight: "100vh", backgroundColor: "#000", color: "#fff", display: "flex", flexDirection: "column", overflowX: "hidden" }}>
        <Starfield />

        {/* Scanlines */}
        <div style={{ position: "fixed", inset: 0, backgroundImage: "repeating-linear-gradient(0deg,transparent,transparent 2px,rgba(0,0,0,0.055) 2px,rgba(0,0,0,0.055) 4px)", pointerEvents: "none", zIndex: 1 }} />

        {/* ── NAV ── */}
        <nav style={{ position: "relative", zIndex: 10, display: "flex", alignItems: "center", justifyContent: "space-between", padding: "28px 52px", borderBottom: "1px solid rgba(255,255,255,0.09)" }}>
          <div style={{ display: "flex", alignItems: "center", gap: 16 }}>
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img
              src="/Alien-Protocol.png"
              alt="Alien Protocol Logo"
              width={44}
              height={44}
              style={{ filter: "drop-shadow(0 0 8px rgba(255,255,255,0.4))" }}
            />
            <span style={{ fontSize: 17, fontWeight: 700, letterSpacing: "0.22em", textTransform: "uppercase", color: "rgba(255,255,255,0.65)" }}>
              Alien <span style={{ color: "#fff", fontWeight: 900 }}>Protocol</span>
            </span>
          </div>
          <a
            href="https://github.com/Alien-Protocol/Alien-Gateway"
            target="_blank"
            rel="noreferrer"
            className="github-btn"
            style={{ fontSize: 13, fontWeight: 600, letterSpacing: "0.2em", textTransform: "uppercase", color: "rgba(255,255,255,0.5)", border: "1px solid rgba(255,255,255,0.25)", padding: "11px 26px", textDecoration: "none", transition: "all 0.2s" }}
          >
            GitHub ↗
          </a>
        </nav>

        {/* ── HERO ── */}
        <main style={{ position: "relative", zIndex: 10, flex: 1, display: "flex", flexDirection: "column", alignItems: "center", justifyContent: "center", textAlign: "center", padding: "80px 32px" }}>

          {/* PNG Logo with pulsing glow */}
          <div style={{ marginBottom: 44, animation: "glow-pulse 3s ease-in-out infinite" }}>
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img
              src="/Alien-Protocol.png"
              alt="Alien Protocol"
              width={170}
              height={170}
              style={{ display: "block" }}
            />
          </div>

          {/* Brand name */}
          <p style={{ fontSize: 15, fontWeight: 700, letterSpacing: "0.55em", textTransform: "uppercase", color: "rgba(255,255,255,0.45)", marginBottom: 30 }}>
            Alien Protocol
          </p>

          {/* COMING SOON — bold & big */}
          <div style={{ display: "inline-flex", alignItems: "center", gap: 14, border: "2px solid rgba(255,255,255,0.55)", padding: "14px 38px", marginBottom: 44, background: "rgba(255,255,255,0.04)" }}>
            <span style={{ width: 10, height: 10, backgroundColor: "#fff", display: "inline-block", borderRadius: "50%", animation: "blink 1.4s infinite" }} />
            <span style={{ fontSize: 22, fontWeight: 900, letterSpacing: "0.45em", textTransform: "uppercase", color: "#fff" }}>
              Coming Soon
            </span>
            <span style={{ width: 10, height: 10, backgroundColor: "#fff", display: "inline-block", borderRadius: "50%", animation: "blink 1.4s infinite 0.7s" }} />
          </div>

          {/* Tagline */}
          <p style={{ fontSize: 18, fontWeight: 500, color: "rgba(255,255,255,0.6)", maxWidth: 500, lineHeight: 2.0, marginBottom: 52, letterSpacing: "0.06em" }}>
            Privacy-preserving username system for Stellar.
            <br />
            Send crypto to{" "}
            <span style={{ color: "#fff", fontWeight: 800 }}>@username</span>
            {" "}— not a 56-char address.
          </p>

          {/* Email notify form */}
          {!submitted ? (
            <form
              onSubmit={(e) => { e.preventDefault(); if (email) setSubmitted(true); }}
              style={{ display: "flex", border: "1px solid rgba(255,255,255,0.3)", width: "100%", maxWidth: 500 }}
            >
              <input
                type="email"
                placeholder="your@email.com"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                style={{ flex: 1, background: "transparent", border: "none", outline: "none", padding: "18px 22px", fontFamily: "'Orbitron', monospace", fontSize: 14, color: "#fff", letterSpacing: "0.08em" }}
              />
              <button
                type="submit"
                className="notify-btn"
                style={{ background: "#fff", color: "#000", border: "none", borderLeft: "1px solid rgba(255,255,255,0.2)", padding: "18px 28px", fontFamily: "'Orbitron', monospace", fontSize: 13, fontWeight: 800, letterSpacing: "0.2em", textTransform: "uppercase", cursor: "pointer", transition: "background 0.2s", whiteSpace: "nowrap" }}
              >
                Notify Me
              </button>
            </form>
          ) : (
            <div style={{ border: "1px solid rgba(255,255,255,0.3)", padding: "18px 44px", fontSize: 15, fontWeight: 700, letterSpacing: "0.3em", textTransform: "uppercase", color: "rgba(255,255,255,0.65)" }}>
              ✓ &nbsp; You&apos;re on the list
            </div>
          )}

          <p style={{ marginTop: 16, fontSize: 12, fontWeight: 400, letterSpacing: "0.25em", textTransform: "uppercase", color: "rgba(255,255,255,0.22)" }}>
            No spam. Launch announcement only.
          </p>
        </main>

        {/* ── FOOTER ── */}
        <footer style={{ position: "relative", zIndex: 10, borderTop: "1px solid rgba(255,255,255,0.09)", padding: "24px 52px", display: "flex", alignItems: "center", justifyContent: "space-between", flexWrap: "wrap", gap: 16 }}>
          <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img src="/Alien-Protocol.png" alt="logo" width={28} height={28} style={{ opacity: 0.5 }} />
            <span style={{ fontSize: 12, fontWeight: 600, letterSpacing: "0.2em", textTransform: "uppercase", color: "rgba(255,255,255,0.28)" }}>
              Alien Protocol © 2025
            </span>
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: 32, fontSize: 12, fontWeight: 600, letterSpacing: "0.2em", textTransform: "uppercase", color: "rgba(255,255,255,0.28)" }}>
            <a href="https://github.com/Alien-Protocol/Alien-Gateway" target="_blank" rel="noreferrer" style={{ color: "rgba(255,255,255,0.28)", textDecoration: "none", transition: "color 0.2s" }}>
              GitHub
            </a>
            <span>Built for Stellar</span>
            <span style={{ display: "flex", alignItems: "center", gap: 8 }}>
              <span style={{ width: 7, height: 7, backgroundColor: "rgba(255,255,255,0.4)", display: "inline-block", borderRadius: "50%", animation: "blink 2s infinite" }} />
              ZK-Powered
            </span>
          </div>
        </footer>
      </div>
    </>
  );
}