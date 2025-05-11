import Taro from '@tarojs/taro';
import { View, Text } from '@tarojs/components';
import { Button } from '@antmjs/vantui'
import { FixedTabbar } from '../../Components/Tabbar';


export default function Welcome() {
    const navigateToList = () => {
        Taro.navigateTo({ url: '/pages/literatureList/index' });
    };

    return (
        <View className='m-2'>
            <View><Text>AI 文献阅读助手</Text></View>
            <View><Text>智能分析，高效阅读</Text></View>

            <Button type='primary' plain hairline block onClick={navigateToList}>
                开始使用
            </Button>
            <FixedTabbar active={0} />
        </View>
    );
}