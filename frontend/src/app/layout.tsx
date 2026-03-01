import type { Metadata } from "next";
import "./globals.css";
import { Header } from "@/components/layout/header";
import { Footer } from "@/components/layout/footer";
import { ThemeProvider } from "@/components/theme-provider";

export const metadata: Metadata = {
  metadataBase: new URL("https://rfc2cn.com"),
  title: {
    default: "RFC中文翻译 - RFC2CN",
    template: "%s | RFC2CN",
  },
  description:
    "RFC2CN 提供高质量的 RFC（Request for Comments）技术文档中文翻译，涵盖网络协议、HTTP、TCP/IP、TLS 等核心互联网标准，支持中英文对照阅读。",
  keywords: [
    "RFC",
    "RFC翻译",
    "RFC中文",
    "Request for Comments",
    "技术文档",
    "网络协议",
    "HTTP",
    "TCP/IP",
    "TLS",
    "互联网标准",
    "中文翻译",
  ],
  authors: [{ name: "RFC2CN", url: "https://rfc2cn.com" }],
  creator: "RFC2CN",
  publisher: "RFC2CN",
  alternates: {
    canonical: "/",
  },
  openGraph: {
    type: "website",
    locale: "zh_CN",
    url: "https://rfc2cn.com",
    siteName: "RFC2CN",
    title: "RFC中文翻译 - RFC2CN",
    description:
      "高质量的 RFC 技术文档中文翻译，涵盖 HTTP、TCP/IP、TLS 等核心互联网标准。",
    images: [
      {
        url: "/og-image.png",
        width: 1200,
        height: 630,
        alt: "RFC2CN - RFC中文翻译",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "RFC中文翻译 - RFC2CN",
    description:
      "高质量的 RFC 技术文档中文翻译，涵盖 HTTP、TCP/IP、TLS 等核心互联网标准。",
    images: ["/og-image.png"],
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-video-preview": -1,
      "max-image-preview": "large",
      "max-snippet": -1,
    },
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="zh-CN" suppressHydrationWarning>
      <body className="font-sans antialiased">
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <div className="min-h-screen flex flex-col">
            <Header />
            <main className="flex-1">{children}</main>
            <Footer />
          </div>
        </ThemeProvider>
      </body>
    </html>
  );
}
