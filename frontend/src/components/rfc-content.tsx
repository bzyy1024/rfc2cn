"use client";

import { useState } from "react";
import type { Rfc, Translation } from "@/lib/api";

interface Props {
  rfc: Rfc;
  translations: Translation[];
}

type ViewMode = "bilingual" | "chinese" | "english";

export function RfcContent({ rfc, translations }: Props) {
  const [viewMode, setViewMode] = useState<ViewMode>("bilingual");

  // 创建翻译映射
  const translationMap = new Map(
    translations.map((t) => [t.section_id, t])
  );

  return (
    <div>
      {/* 视图切换 */}
      <div className="sticky top-14 z-10 bg-background border-b py-3 mb-6">
        <div className="flex items-center gap-2">
          <span className="text-sm text-muted-foreground mr-2">显示模式:</span>
          <button
            onClick={() => setViewMode("bilingual")}
            className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
              viewMode === "bilingual"
                ? "bg-primary text-primary-foreground"
                : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }`}
          >
            中英对照
          </button>
          <button
            onClick={() => setViewMode("chinese")}
            className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
              viewMode === "chinese"
                ? "bg-primary text-primary-foreground"
                : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }`}
          >
            仅中文
          </button>
          <button
            onClick={() => setViewMode("english")}
            className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
              viewMode === "english"
                ? "bg-primary text-primary-foreground"
                : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }`}
          >
            仅英文
          </button>
        </div>
      </div>

      {/* 内容区域 */}
      <div className="rfc-content">
        {translations.length > 0 ? (
          <div className="space-y-6">
            {translations.map((section) => (
              <div
                key={section.section_id}
                id={`section-${section.section_id}`}
                className={
                  viewMode === "bilingual" ? "bilingual-section" : ""
                }
              >
                {/* 英文 */}
                {(viewMode === "bilingual" || viewMode === "english") && (
                  <div
                    className={
                      viewMode === "bilingual"
                        ? "original whitespace-pre-wrap"
                        : "whitespace-pre-wrap"
                    }
                  >
                    {section.original_text}
                  </div>
                )}

                {/* 中文 */}
                {(viewMode === "bilingual" || viewMode === "chinese") && (
                  <div
                    className={
                      viewMode === "bilingual"
                        ? "translated whitespace-pre-wrap"
                        : "whitespace-pre-wrap"
                    }
                  >
                    {section.translated_text || (
                      <span className="text-muted-foreground italic">
                        [暂无翻译]
                      </span>
                    )}
                  </div>
                )}
              </div>
            ))}
          </div>
        ) : (
          // 如果没有分段翻译，显示原文
          <div className="whitespace-pre-wrap">
            {rfc.original_text || "暂无内容"}
          </div>
        )}
      </div>
    </div>
  );
}
