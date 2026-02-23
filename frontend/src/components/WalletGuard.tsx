// export default function WalletGuard() {
//   return (
//     <div className="flex items-center justify-center h-full">
//       <div className="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-blue-500"></div>
//     </div>
//   );
// }

"use client";

import { useWallet } from "@/context/WalletContext";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function WalletGuard({ children }: { children: React.ReactNode }) {
  const { isConnected, isLoading } = useWallet();
  const router = useRouter();

  useEffect(() => {
    if (!isLoading && !isConnected) {
      router.replace("/");
    }
  }, [isConnected, isLoading, router]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-blue-500" />
      </div>
    );
  }

  if (!isConnected) return null;

  return <>{children}</>;
}