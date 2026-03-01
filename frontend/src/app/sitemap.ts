import { MetadataRoute } from "next";
import { getRfcsPaginated } from "@/lib/api";

const BASE_URL = "https://rfc2cn.com";

export default async function sitemap(): Promise<MetadataRoute.Sitemap> {
  const staticPages: MetadataRoute.Sitemap = [
    {
      url: BASE_URL,
      lastModified: new Date(),
      changeFrequency: "daily",
      priority: 1.0,
    },
    {
      url: `${BASE_URL}/rfcs`,
      lastModified: new Date(),
      changeFrequency: "daily",
      priority: 0.9,
    },
    {
      url: `${BASE_URL}/about`,
      lastModified: new Date(),
      changeFrequency: "monthly",
      priority: 0.5,
    },
  ];

  try {
    // 获取所有 RFC 列表（最多 2000 条）
    const data = await getRfcsPaginated(1, 2000);
    const rfcPages: MetadataRoute.Sitemap = data.rfcs.map((rfc) => ({
      url: `${BASE_URL}/rfc/${rfc.rfc_number}`,
      lastModified: new Date(rfc.created_at),
      changeFrequency: "weekly" as const,
      priority: 0.7,
    }));
    return [...staticPages, ...rfcPages];
  } catch {
    return staticPages;
  }
}
