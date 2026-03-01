import { ImageResponse } from "next/og";

export const size = { width: 32, height: 32 };
export const contentType = "image/png";

export default function Icon() {
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
          borderRadius: "7px",
          fontFamily: "sans-serif",
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            gap: "0px",
          }}
        >
          <span
            style={{
              color: "white",
              fontSize: "13px",
              fontWeight: 700,
              lineHeight: 1.1,
              letterSpacing: "-0.5px",
            }}
          >
            RFC
          </span>
          <span
            style={{
              color: "#bfdbfe",
              fontSize: "8px",
              fontWeight: 600,
              lineHeight: 1,
              letterSpacing: "0.5px",
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
