import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Allow images from external sources (useful for kit cover images)
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "**.edukit.io",
      },
    ],
  },
};

export default nextConfig;
