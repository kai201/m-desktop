import { Outlet } from "react-router-dom";
import { Layout, Menu } from "antd";
import {
  HomeOutlined,
  MessageOutlined,
  CalendarOutlined,
  TeamOutlined,
  SettingOutlined,
} from "@ant-design/icons";

const { Header, Sider, Content } = Layout;

export default function AppLayout() {
  return (
    <Layout className="h-screen">
      {/* 左侧导航栏 */}
      <Sider theme="dark" width={64}>
        <div className="h-8 bg-[#2a2e32] m-2 rounded" />
        <Menu
          theme="dark"
          mode="inline"
          defaultSelectedKeys={["1"]}
          inlineCollapsed={true}
          items={[
            {
              key: "1",
              icon: <HomeOutlined />,
              label: "首页",
            },
            {
              key: "2",
              icon: <MessageOutlined />,
              label: "消息",
            },
            {
              key: "3",
              icon: <CalendarOutlined />,
              label: "日历",
            },
            {
              key: "4",
              icon: <TeamOutlined />,
              label: "通讯录",
            },
            {
              key: "5",
              icon: <SettingOutlined />,
              label: "设置",
            },
          ]}
        />
      </Sider>

      <Layout>
        {/* 顶部工具栏 */}
        <Header className="bg-[#2a2e32] px-4 flex items-center justify-between">
          <div className="text-white text-lg font-medium">企业微信风格布局</div>
        </Header>

        {/* 主内容区域 */}
        <Content className="p-4 overflow-auto bg-[#f0f2f5]">
          <Outlet />
        </Content>
      </Layout>
    </Layout>
  );
}
