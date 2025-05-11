import Taro from "@tarojs/taro";
import { View } from "@tarojs/components";
import { Tabbar, TabbarItem } from "@antmjs/vantui";

interface FixedTabbarProps {
    active: number | string;
}

const pages = [
    "/pages/welcome/index",
    "/pages/literatureList/index",
    // "pages/literatureDetail/index",
];

export function FixedTabbar({ active }: FixedTabbarProps) {
    return (
        <View>
            <Tabbar
                active={active}
                onChange={(e) => {
                    switch (e.detail) {
                        case 0:
                            Taro.navigateTo({ url: pages[0] });
                            break;
                        case 1:
                            Taro.navigateTo({ url: pages[1] });
                            break;
                    }
                }}
                safeAreaInsetBottom={true}
                activeColor='#0f7f6c'
            >
                <TabbarItem icon="wap-home-o">Home</TabbarItem>
                <TabbarItem icon="orders-o">文献</TabbarItem>
                <TabbarItem icon="user-o">我的</TabbarItem>
            </Tabbar>
        </View>
    )
}