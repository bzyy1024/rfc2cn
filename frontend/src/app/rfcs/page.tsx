import { Metadata } from "next";
import { getRfcsPaginated } from "@/lib/api";
import { RfcListClient } from "@/components/rfc-list-client";

export const metadata: Metadata = {
  title: "RFC文档列表",
  description: "浏览所有已翻译的 RFC 技术文档，涵盖 HTTP、TCP/IP、TLS、DNS 等互联网核心协议标准的中文翻译。",
  keywords: ["RFC列表", "RFC文档", "网络协议标准", "RFC中文翻译", "互联网协议"],
  alternates: {
    canonical: "/rfcs",
  },
  openGraph: {
    title: "RFC文档列表 | RFC2CN",
    description: "浏览所有已翻译的 RFC 技术文档，涵盖 HTTP、TCP/IP、TLS、DNS 等互联网核心协议标准。",
    type: "website",
  },
};

// 禁用静态生成，强制动态渲染
export const dynamic = 'force-dynamic';

interface Props {
  searchParams: { page?: string };
}

export default async function RfcsPage({ searchParams }: Props) {
  const page = parseInt(searchParams.page || "1", 10);
  
  let data = null;
  let error = null;
  
  try {
    data = await getRfcsPaginated(page, 20);
  } catch (err) {
    console.error("Failed to fetch RFCs:", err);
    error = "加载失败，请稍后重试";
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-8">RFC文档列表</h1>
      <RfcListClient initialData={data} initialError={error} />
    </div>
  );
}
