import { ImageResponse } from "next/og";

export const size = { width: 180, height: 180 };
export const contentType = "image/png";

export default function AppleIcon() {
  return new ImageResponse(
    (
      <div
        style={{
          background: "linear-gradient(135deg, #4f46e5 0%, #2563eb 100%)",
          width: "100%",
          height: "100%",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          borderRadius: "40px",
          fontFamily: "sans-serif",
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            gap: "4px",
          }}
        >
          <span
            style={{
              color: "white",
              fontSize: "72px",
              fontWeight: 700,
              lineHeight: 1,
              letterSpacing: "-2px",
            }}
          >
            RFC
          </span>
          <span
            style={{
              color: "#bfdbfe",
              fontSize: "40px",
              fontWeight: 600,
              lineHeight: 1,
              letterSpacing: "4px",
            }}
          >
            中文
          </span>
        </div>
      </div>
    ),
    { ...size }
  );
}
