import type { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'repomd — The Codebase Context Compiler',
  description: 'Any repo. One command. Perfect context. Aggressively structured for LLMs.',
  viewport: 'width=device-width, initial-scale=1, maximum-scale=1',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <header className="header">
          <a href="/" className="logo">
            <span className="glitch-text" data-text="repomd">repomd</span>
          </a>
          <nav className="nav-links">
            <a href="https://github.com/repomd/repomd" target="_blank" rel="noopener noreferrer">GitHub</a>
            <a href="/docs">Docs</a>
            <a href="/api-reference">API</a>
          </nav>
        </header>
        {children}
        <footer className="footer">
          INITIATIVE OF REPOMD.DEV SYSTEM &copy; {new Date().getFullYear()} // OPEN SOURCE. MIT LICENSED.
        </footer>
      </body>
    </html>
  );
}
