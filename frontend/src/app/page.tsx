import Link from "next/link";
import { SearchBox } from "@/components/search-box";
import { RfcListClient } from "@/components/rfc-list-client";
import { getRfcsPaginated } from "@/lib/api";

// 禁用静态生成，强制动态渲染
export const dynamic = 'force-dynamic';

export default async function Home() {
  let data = null;
  let error = null;
  
  try {
    // 首页只显示前10条，不需要分页
    data = await getRfcsPaginated(1, 10);
  } catch (err) {
    console.error("Failed to fetch RFCs:", err);
    error = "加载失败，请稍后重试";
  }

  return (
    <div className="container mx-auto px-4 py-8">
      {/* Hero Section */}
      <section className="text-center mb-12">
        <h1 className="text-4xl font-bold mb-4">RFC中文翻译</h1>
        <p className="text-lg text-muted-foreground mb-8">
          提供高质量的RFC技术文档中文翻译，支持中英文对照阅读
        </p>
        <SearchBox />
      </section>

      {/* RFC List Section */}
      <section>
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-2xl font-semibold">最新翻译</h2>
          <Link 
            href="/rfcs" 
            className="text-sm text-primary hover:underline"
          >
            查看全部 →
          </Link>
        </div>
        <RfcListClient initialData={data} initialError={error} showPagination={false} />
      </section>
    </div>
  );
}
