import { useToastStore } from "./toastStore";

export function ToastContainer() {
  const { toasts, removeToast } = useToastStore();

  if (toasts.length === 0) return null;

  return (
    <div style={{
      position: "fixed",
      bottom: "20px",
      right: "20px",
      display: "flex",
      flexDirection: "column",
      gap: "10px",
      zIndex: 9999,
    }}>
      {toasts.map((toast) => {
        let bgColor = "var(--color-surface)";
        let borderColor = "var(--color-border)";
        
        if (toast.type === "success") {
          bgColor = "rgba(46, 125, 50, 0.9)";
          borderColor = "#4CAF50";
        } else if (toast.type === "error") {
          bgColor = "rgba(198, 40, 40, 0.9)";
          borderColor = "#F44336";
        } else if (toast.type === "warning") {
          bgColor = "rgba(239, 108, 0, 0.9)";
          borderColor = "#FF9800";
        } else if (toast.type === "info") {
          bgColor = "rgba(21, 101, 192, 0.9)";
          borderColor = "#2196F3";
        }

        return (
          <div key={toast.id} style={{
            backgroundColor: bgColor,
            color: "white",
            padding: "12px 16px",
            borderRadius: "6px",
            borderLeft: `4px solid ${borderColor}`,
            boxShadow: "0 4px 12px rgba(0,0,0,0.3)",
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            minWidth: "250px",
            animation: "slideIn 0.3s ease-out"
          }}>
            <span style={{ fontSize: "14px", fontWeight: "500" }}>{toast.message}</span>
            <button 
              onClick={() => removeToast(toast.id)}
              style={{
                background: "transparent",
                border: "none",
                color: "rgba(255,255,255,0.7)",
                cursor: "pointer",
                padding: "4px",
                marginLeft: "12px",
                fontSize: "16px"
              }}
            >
              ×
            </button>
          </div>
        );
      })}
    </div>
  );
}
