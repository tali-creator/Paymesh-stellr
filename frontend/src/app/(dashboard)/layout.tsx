import { Navbar } from "@/components/Navbar";
import WalletGuard from "@/components/WalletGuard"; 

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <>
      <div
        className="min-h-screen w-full fixed inset-0 -z-10"
        style={{
          backgroundImage: 'url("/Bg 1.svg")',
          backgroundSize: "cover",
          backgroundPosition: "center",
          backgroundRepeat: "no-repeat",
          backgroundAttachment: "fixed",
        }}
      />
      <Navbar />
      <WalletGuard>
        <div className="min-h-screen pt-24 sm:pt-28 lg:pt-32 pb-8 sm:pb-12 px-3 sm:px-4 lg:px-8 max-w-5xl mx-auto relative z-10 w-full overflow-x-hidden">
          {children}
        </div>
      </WalletGuard>
    </>
  );
}
