import { Metadata } from "next";
import Link from "next/link";
import { searchRfcs } from "@/lib/api";
import { SearchBox } from "@/components/search-box";
import { Pagination } from "@/components/pagination";
import { FileText, Clock } from "lucide-react";

export const metadata: Metadata = {
  title: "搜索RFC文档",
  description: "搜索 RFC2CN 收录的 RFC 技术文档，支持按 RFC 编号、标题等关键词检索。",
  robots: {
    index: false,
    follow: true,
  },
};

interface Props {
  searchParams: { q?: string; page?: string };
}

export default async function SearchPage({ searchParams }: Props) {
  const query = searchParams.q || "";
  const page = parseInt(searchParams.page || "1");

  let result = null;
  if (query) {
    try {
      result = await searchRfcs(query, page, 20);
    } catch (error) {
      console.error("Search failed:", error);
    }
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-8">搜索RFC文档</h1>

      <div className="mb-8">
        <SearchBox />
      </div>

      {query && (
        <div className="mb-4 text-muted-foreground">
          搜索 &ldquo;{query}&rdquo;
          {result && ` - 找到 ${result.total} 个结果`}
        </div>
      )}

      {result && result.rfcs.length > 0 ? (
        <>
          <div className="grid gap-4">
            {result.rfcs.map((rfc) => (
              <Link
                key={rfc.id}
                href={`/rfc/${rfc.rfc_number}`}
                className="block p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <h3 className="font-semibold">RFC {rfc.rfc_number}</h3>
                    <p className="text-lg mt-1">{rfc.title}</p>
                    {rfc.abstract && (
                      <p className="text-sm text-muted-foreground mt-2 line-clamp-2">
                        {rfc.abstract}
                      </p>
                    )}
                  </div>
                </div>
                <div className="flex items-center gap-4 mt-3 text-xs text-muted-foreground">
                  <span className="flex items-center">
                    <Clock className="h-3 w-3 mr-1" />
                    {new Date(rfc.created_at).toLocaleDateString("zh-CN")}
                  </span>
                </div>
              </Link>
            ))}
          </div>

          {/* 分页 */}
          <Pagination
            currentPage={result.page}
            totalPages={result.total_pages}
            baseUrl="/search"
            queryParams={{ q: query }}
          />
        </>
      ) : query ? (
        <div className="text-center py-12 text-muted-foreground">
          <FileText className="h-12 w-12 mx-auto mb-4 opacity-50" />
          <p>未找到相关RFC文档</p>
        </div>
      ) : (
        <div className="text-center py-12 text-muted-foreground">
          <p>请输入搜索关键词</p>
        </div>
      )}
    </div>
  );
}
