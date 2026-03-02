// API_URL is a server-side-only var used for internal Docker network requests (SSR).
// NEXT_PUBLIC_API_URL is the browser-facing public URL.
const API_BASE_URL =
  (typeof window === 'undefined' ? process.env.API_URL : undefined) ||
  process.env.NEXT_PUBLIC_API_URL ||
  'http://localhost:8080';

export interface Rfc {
  id: number;
  rfc_number: number;
  title: string;
  original_text?: string;
  status: string;
  abstract?: string;
  publish_date?: string;
  created_at: string;
  updated_at: string;
}

export interface RfcListItem {
  id: number;
  rfc_number: number;
  title: string;
  status: string;
  abstract?: string;
  publish_date?: string;
  created_at: string;
}

// Tag interfaces removed. API functions below are deprecated.

export interface Translation {
  id: number;
  rfc_id: number;
  section_id: string;
  original_text: string;
  translated_text?: string;
  reviewed: boolean;
  created_at: string;
  updated_at: string;
}

export interface SearchResponse {
  rfcs: RfcListItem[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

// 获取RFC列表
export async function getRfcs(): Promise<RfcListItem[]> {
  const res = await fetch(`${API_BASE_URL}/api/rfcs`, {
    next: { revalidate: 60 },
  });
  if (!res.ok) throw new Error('Failed to fetch RFCs');
  return res.json();
}

// 搜索RFC
export async function searchRfcs(
  query?: string,
  page: number = 1,
  perPage: number = 20
): Promise<SearchResponse> {
  const params = new URLSearchParams();
  if (query) params.set('q', query);
  params.set('page', page.toString());
  params.set('per_page', perPage.toString());

  const res = await fetch(`${API_BASE_URL}/api/rfcs/search?${params}`, {
    next: { revalidate: 60 },
  });
  if (!res.ok) throw new Error('Failed to search RFCs');
  return res.json();
}

// 获取RFC列表（带分页）
export async function getRfcsPaginated(
  page: number = 1,
  perPage: number = 20
): Promise<SearchResponse> {
  const params = new URLSearchParams();
  params.set('page', page.toString());
  params.set('per_page', perPage.toString());

  const url = `${API_BASE_URL}/api/rfcs/search?${params}`;
  
  const res = await fetch(url, {
    next: { revalidate: 60 }, // ISR: 60秒后重新验证
  });
  
  if (!res.ok) {
    throw new Error(`Failed to fetch RFCs: ${res.status}`);
  }
  
  return res.json();
}

// 获取RFC详情
export async function getRfc(number: number): Promise<Rfc> {
  const res = await fetch(`${API_BASE_URL}/api/rfcs/${number}`, {
    next: { revalidate: 300 },
  });
  if (!res.ok) throw new Error('Failed to fetch RFC');
  return res.json();
}

// 获取RFC翻译
export async function getRfcTranslations(number: number): Promise<Translation[]> {
  const res = await fetch(`${API_BASE_URL}/api/rfcs/${number}/translations`, {
    next: { revalidate: 300 },
  });
  if (!res.ok) throw new Error('Failed to fetch translations');
  return res.json();
}

// 获取相邻的RFC（上一个和下一个）
export async function getAdjacentRfcs(currentNumber: number): Promise<{
  previous: RfcListItem | null;
  next: RfcListItem | null;
}> {
  try {
    const allRfcs = await getRfcs();
    const currentIndex = allRfcs.findIndex(rfc => rfc.rfc_number === currentNumber);
    
    if (currentIndex === -1) {
      return { previous: null, next: null };
    }
    
    return {
      previous: currentIndex > 0 ? allRfcs[currentIndex - 1] : null,
      next: currentIndex < allRfcs.length - 1 ? allRfcs[currentIndex + 1] : null,
    };
  } catch (error) {
    console.error('Failed to fetch adjacent RFCs:', error);
    return { previous: null, next: null };
  }
}
