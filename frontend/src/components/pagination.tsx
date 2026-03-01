"use client";

import Link from "next/link";
import { ChevronLeft, ChevronRight } from "lucide-react";

interface PaginationProps {
  currentPage: number;
  totalPages: number;
  baseUrl: string;
  queryParams?: Record<string, string>;
}

export function Pagination({
  currentPage,
  totalPages,
  baseUrl,
  queryParams = {},
}: PaginationProps) {
  if (totalPages <= 1) return null;

  const buildUrl = (page: number) => {
    const params = new URLSearchParams({ ...queryParams, page: page.toString() });
    const queryString = params.toString();
    return queryString ? `${baseUrl}?${queryString}` : `${baseUrl}?page=${page}`;
  };

  // 生成页码数组
  const getPageNumbers = () => {
    const pages: (number | string)[] = [];
    const maxVisible = 7; // 最多显示7个页码按钮

    if (totalPages <= maxVisible) {
      // 总页数少于最大显示数，显示所有页码
      for (let i = 1; i <= totalPages; i++) {
        pages.push(i);
      }
    } else {
      // 总页数多，需要智能显示
      if (currentPage <= 3) {
        // 当前页靠前
        for (let i = 1; i <= 5; i++) {
          pages.push(i);
        }
        pages.push("...");
        pages.push(totalPages);
      } else if (currentPage >= totalPages - 2) {
        // 当前页靠后
        pages.push(1);
        pages.push("...");
        for (let i = totalPages - 4; i <= totalPages; i++) {
          pages.push(i);
        }
      } else {
        // 当前页在中间
        pages.push(1);
        pages.push("...");
        for (let i = currentPage - 1; i <= currentPage + 1; i++) {
          pages.push(i);
        }
        pages.push("...");
        pages.push(totalPages);
      }
    }

    return pages;
  };

  const pageNumbers = getPageNumbers();

  return (
    <div className="flex items-center justify-center gap-1 mt-8">
      {/* 上一页 */}
      {currentPage > 1 ? (
        <Link
          href={buildUrl(currentPage - 1)}
          className="inline-flex items-center justify-center w-9 h-9 rounded-md border bg-background hover:bg-accent hover:text-accent-foreground transition-colors"
          aria-label="上一页"
        >
          <ChevronLeft className="h-4 w-4" />
        </Link>
      ) : (
        <button
          disabled
          className="inline-flex items-center justify-center w-9 h-9 rounded-md border bg-background opacity-50 cursor-not-allowed"
          aria-label="上一页"
        >
          <ChevronLeft className="h-4 w-4" />
        </button>
      )}

      {/* 页码 */}
      <div className="flex items-center gap-1">
        {pageNumbers.map((page, index) => {
          if (page === "...") {
            return (
              <span
                key={`ellipsis-${index}`}
                className="inline-flex items-center justify-center w-9 h-9 text-muted-foreground"
              >
                ...
              </span>
            );
          }

          const pageNum = page as number;
          const isActive = pageNum === currentPage;

          return (
            <Link
              key={pageNum}
              href={buildUrl(pageNum)}
              className={`inline-flex items-center justify-center w-9 h-9 rounded-md border text-sm font-medium transition-colors ${
                isActive
                  ? "bg-primary text-primary-foreground border-primary"
                  : "bg-background hover:bg-accent hover:text-accent-foreground"
              }`}
              aria-label={`第 ${pageNum} 页`}
              aria-current={isActive ? "page" : undefined}
            >
              {pageNum}
            </Link>
          );
        })}
      </div>

      {/* 下一页 */}
      {currentPage < totalPages ? (
        <Link
          href={buildUrl(currentPage + 1)}
          className="inline-flex items-center justify-center w-9 h-9 rounded-md border bg-background hover:bg-accent hover:text-accent-foreground transition-colors"
          aria-label="下一页"
        >
          <ChevronRight className="h-4 w-4" />
        </Link>
      ) : (
        <button
          disabled
          className="inline-flex items-center justify-center w-9 h-9 rounded-md border bg-background opacity-50 cursor-not-allowed"
          aria-label="下一页"
        >
          <ChevronRight className="h-4 w-4" />
        </button>
      )}

      {/* 页码信息 */}
      <span className="ml-4 text-sm text-muted-foreground whitespace-nowrap">
        {currentPage} / {totalPages}
      </span>
    </div>
  );
}
