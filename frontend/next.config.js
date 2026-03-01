/** @type {import('next').NextConfig} */
const nextConfig = {
  // 启用静态导出以支持SEO
  output: 'standalone',
  
  // 配置API代理
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: `${process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080'}/api/:path*`,
      },
    ];
  },

  // SEO相关配置
  poweredByHeader: false,
  
  // 图片优化
  images: {
    domains: [],
  },
};

module.exports = nextConfig;
