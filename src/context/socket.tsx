import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
} from "react";
import io, { Socket } from "socket.io-client";

type SocketContextType = {
  socket: Socket;
  useSocketEvent: <T = any>(event: string, handler: (data: T) => void) => void;
};

const SocketContext = createContext<SocketContextType | null>(null);

export function SocketProvider({
  children,
  url,
  opts = {},
}: {
  children: React.ReactNode;
  url: string;
  opts?: any;
}) {
  const socket = useMemo(
    () =>
      io(url, {
        ...opts,
        autoConnect: false,
        reconnection: true,
        reconnectionAttempts: 5,
        reconnectionDelay: 3000,
        timeout: 10000,
        transports: ["websocket"],
      }),
    [url, opts]
  );

  // 全局事件处理器注册系统
  const useSocketEvent = <T,>(event: string, handler: (data: T) => void) => {
    const stableHandler = useEvent(handler);

    useEffect(() => {
      socket.on(event, stableHandler);
      return () => {
        socket.off(event, stableHandler);
      };
    }, [event, stableHandler]);
  };

  // 组件卸载时断开连接
  useEffect(() => {
    return () => {
      if (socket.connected) socket.disconnect();
    };
  }, []);

  return (
    <SocketContext.Provider value={{ socket, useSocketEvent }}>
      {children}
    </SocketContext.Provider>
  );
}

// 自定义 hook
export function useSocket() {
  const context = useContext(SocketContext);
  if (!context) {
    throw new Error("useSocket must be used within a SocketProvider");
  }
  return context;
}

// 实现 useEvent（兼容 React 18-）
function useEvent<T extends (...args: any[]) => any>(handler: T) {
  const handlerRef = useRef<T>(handler);
  handlerRef.current = handler;

  return useCallback((...args: Parameters<T>) => {
    return handlerRef.current(...args);
  }, []) as T;
}
