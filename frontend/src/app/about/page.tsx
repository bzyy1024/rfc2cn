import { Metadata } from "next";

export const metadata: Metadata = {
  title: "关于我们",
  description: "RFC2CN 是一个专注于 RFC 技术文档中文翻译的开源项目，致力于让中文用户更好地阅读和理解互联网标准文档。",
  alternates: {
    canonical: "/about",
  },
  openGraph: {
    title: "关于 RFC2CN",
    description: "RFC2CN 是一个专注于 RFC 技术文档中文翻译的开源项目。",
    type: "website",
  },
};

export default function AboutPage() {
  return (
    <div className="container mx-auto px-4 py-8 max-w-3xl">
      <h1 className="text-3xl font-bold mb-8">关于 RFC2CN</h1>

      <div className="prose prose-gray dark:prose-invert max-w-none">
        <section className="mb-8">
          <h2 className="text-2xl font-semibold mb-4">项目介绍</h2>
          <p className="text-muted-foreground mb-4">
            RFC2CN 是一个专注于将 RFC（Request for Comments）技术文档翻译成中文的网站。
            我们的目标是为中文用户提供高质量的RFC文档翻译，降低技术文档的阅读门槛。
          </p>
        </section>

        <section className="mb-8">
          <h2 className="text-2xl font-semibold mb-4">什么是RFC？</h2>
          <p className="text-muted-foreground mb-4">
            RFC（Request for Comments）是互联网工程任务组（IETF）发布的一系列技术文档，
            描述了互联网的各种协议、方法和研究。许多重要的网络协议都在RFC中定义，
            如HTTP、TCP/IP、DNS等。
          </p>
        </section>

        <section className="mb-8">
          <h2 className="text-2xl font-semibold mb-4">功能特点</h2>
          <ul className="list-disc list-inside text-muted-foreground space-y-2">
            <li>中英文对照阅读，方便理解原文</li>
            <li>支持仅中文/仅英文模式切换</li>
            <li>按标签分类，快速查找相关文档</li>
            <li>全文搜索功能</li>
            <li>SEO优化，方便搜索引擎收录</li>
          </ul>
        </section>

        <section className="mb-8">
          <h2 className="text-2xl font-semibold mb-4">技术栈</h2>
          <ul className="list-disc list-inside text-muted-foreground space-y-2">
            <li>前端：Next.js + Tailwind CSS + shadcn/ui</li>
            <li>后端：Rust + Axum + PostgreSQL</li>
            <li>翻译：Ollama (本地AI翻译)</li>
          </ul>
        </section>

        <section>
          <h2 className="text-2xl font-semibold mb-4">贡献翻译</h2>
          <p className="text-muted-foreground mb-4">
            如果您发现翻译有误或想要贡献新的翻译，欢迎联系我们或提交Issue。
          </p>
        </section>
      </div>
    </div>
  );
}
