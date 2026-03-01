import { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { getRfc, getRfcTranslations, getAdjacentRfcs } from "@/lib/api";
import { RfcContent } from "@/components/rfc-content";
import { ArrowLeft, ExternalLink, ChevronLeft, ChevronRight } from "lucide-react";

interface Props {
  params: { number: string };
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  try {
    const rfc = await getRfc(parseInt(params.number));
    const description =
      rfc.abstract ||
      `RFC ${rfc.rfc_number} ${rfc.title} 中文翻译，由 RFC2CN 提供高质量翻译。`;
    return {
      title: `RFC ${rfc.rfc_number} - ${rfc.title}`,
      description,
      keywords: [
        `RFC ${rfc.rfc_number}`,
        rfc.title,
        "RFC翻译",
        "RFC中文",
        "技术文档",
      ],
      alternates: {
        canonical: `/rfc/${rfc.rfc_number}`,
      },
      openGraph: {
        title: `RFC ${rfc.rfc_number} - ${rfc.title}`,
        description,
        type: "article",
        url: `https://rfc2cn.com/rfc/${rfc.rfc_number}`,
        publishedTime: rfc.publish_date || rfc.created_at,
        modifiedTime: rfc.updated_at,
      },
      twitter: {
        card: "summary",
        title: `RFC ${rfc.rfc_number} - ${rfc.title}`,
        description,
      },
    };
  } catch {
    return {
      title: "RFC Not Found",
    };
  }
}

export default async function RfcPage({ params }: Props) {
  const rfcNumber = parseInt(params.number);
  
  if (isNaN(rfcNumber)) {
    notFound();
  }

  let rfc;
  let translations;
  let adjacentRfcs;
  
  try {
    [rfc, translations, adjacentRfcs] = await Promise.all([
      getRfc(rfcNumber),
      getRfcTranslations(rfcNumber),
      getAdjacentRfcs(rfcNumber),
    ]);
  } catch (error) {
    notFound();
  }

  return (
    <div className="container mx-auto px-4 py-8">
      {/* JSON-LD 结构化数据 */}
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{
          __html: JSON.stringify({
            "@context": "https://schema.org",
            "@type": "TechArticle",
            headline: `RFC ${rfc.rfc_number} - ${rfc.title}`,
            description: rfc.abstract || `RFC ${rfc.rfc_number} 中文翻译`,
            url: `https://rfc2cn.com/rfc/${rfc.rfc_number}`,
            inLanguage: "zh-CN",
            datePublished: rfc.publish_date || rfc.created_at,
            dateModified: rfc.updated_at,
            publisher: {
              "@type": "Organization",
              name: "RFC2CN",
              url: "https://rfc2cn.com",
            },
            isPartOf: {
              "@type": "WebSite",
              name: "RFC2CN",
              url: "https://rfc2cn.com",
            },
          }),
        }}
      />

      {/* 导航 */}
      <div className="mb-6 flex items-center justify-between">
        <Link
          href="/rfcs"
          className="inline-flex items-center text-sm text-muted-foreground hover:text-foreground"
        >
          <ArrowLeft className="h-4 w-4 mr-1" />
          返回列表
        </Link>
        
        <div className="flex items-center gap-2">
          {adjacentRfcs.previous ? (
            <Link
              href={`/rfc/${adjacentRfcs.previous.rfc_number}`}
              className="inline-flex items-center px-3 py-1.5 text-sm rounded-md border bg-background hover:bg-accent transition-colors"
              title={`RFC ${adjacentRfcs.previous.rfc_number} - ${adjacentRfcs.previous.title}`}
            >
              <ChevronLeft className="h-4 w-4 mr-1" />
              上一个
            </Link>
          ) : (
            <span className="inline-flex items-center px-3 py-1.5 text-sm text-muted-foreground cursor-not-allowed">
              <ChevronLeft className="h-4 w-4 mr-1" />
              上一个
            </span>
          )}
          
          {adjacentRfcs.next ? (
            <Link
              href={`/rfc/${adjacentRfcs.next.rfc_number}`}
              className="inline-flex items-center px-3 py-1.5 text-sm rounded-md border bg-background hover:bg-accent transition-colors"
              title={`RFC ${adjacentRfcs.next.rfc_number} - ${adjacentRfcs.next.title}`}
            >
              下一个
              <ChevronRight className="h-4 w-4 ml-1" />
            </Link>
          ) : (
            <span className="inline-flex items-center px-3 py-1.5 text-sm text-muted-foreground cursor-not-allowed">
              下一个
              <ChevronRight className="h-4 w-4 ml-1" />
            </span>
          )}
        </div>
      </div>

      {/* 头部信息 */}
      <header className="mb-8">
        <div className="flex items-center gap-2 text-sm text-muted-foreground mb-2">
          <span>RFC {rfc.rfc_number}</span>
          <span>•</span>
          <span
            className={`px-2 py-0.5 rounded text-xs ${
              rfc.status === "completed"
                ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-100"
                : "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-100"
            }`}
          >
            {rfc.status === "completed" ? "已翻译" : "部分翻译"}
          </span>
        </div>
        
        <h1 className="text-3xl font-bold mb-4">{rfc.title}</h1>
        
        {rfc.abstract && (
          <p className="text-muted-foreground mb-4">{rfc.abstract}</p>
        )}

        {/* 标签已移除 */}

        {/* 外部链接 */}
        <a
          href={`https://www.rfc-editor.org/rfc/rfc${rfc.rfc_number}.html`}
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center text-sm text-primary hover:underline"
        >
          <ExternalLink className="h-4 w-4 mr-1" />
          查看原文 (IETF)
        </a>
      </header>

      {/* 内容 */}
      <RfcContent rfc={rfc} translations={translations} />
    </div>
  );
}
