"use client";

import Link from "next/link";
import { FileText, Clock } from "lucide-react";
import { Pagination } from "@/components/pagination";
import type { SearchResponse } from "@/lib/api";

interface Props {
  initialData: SearchResponse | null;
  initialError: string | null;
  showPagination?: boolean; // 新增：控制是否显示分页
}

export function RfcListClient({ initialData, initialError, showPagination = true }: Props) {
  if (initialError) {
    return (
      <div className="text-center py-12">
        <div className="text-red-500 mb-4">
          <p className="font-semibold">加载失败</p>
          <p className="text-sm">{initialError}</p>
        </div>
        <button
          onClick={() => window.location.reload()}
          className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
        >
          重新加载
        </button>
      </div>
    );
  }

  if (!initialData || initialData.rfcs.length === 0) {
    return (
      <div className="text-center py-12 text-muted-foreground">
        <FileText className="h-12 w-12 mx-auto mb-4 opacity-50" />
        <p>暂无RFC文档</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="text-sm text-muted-foreground">
        共 {initialData.total} 个RFC文档
      </div>
      
      <div className="grid gap-4">
        {initialData.rfcs.map((rfc) => (
          <Link
            key={rfc.id}
            href={`/rfc/${rfc.rfc_number}`}
            className="block p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
          >
            <div className="flex items-start justify-between">
              <div className="flex-1">
                <h3 className="font-semibold">
                  RFC {rfc.rfc_number}
                </h3>
                <p className="text-lg mt-1">{rfc.title}</p>
                {rfc.abstract && (
                  <p className="text-sm text-muted-foreground mt-2 line-clamp-2">
                    {rfc.abstract}
                  </p>
                )}
              </div>
              <div className="ml-4 flex-shrink-0">
                <span
                  className={`inline-flex items-center px-2 py-1 rounded text-xs font-medium ${
                    rfc.status === "completed"
                      ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-100"
                      : rfc.status === "translating"
                      ? "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-100"
                      : "bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-100"
                  }`}
                >
                  {rfc.status === "completed"
                    ? "已翻译"
                    : rfc.status === "translating"
                    ? "翻译中"
                    : "待翻译"}
                </span>
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

      {showPagination && initialData.total_pages > 1 && (
        <Pagination
          currentPage={initialData.page}
          totalPages={initialData.total_pages}
          baseUrl="/rfcs"
        />
      )}
    </div>
  );
}
