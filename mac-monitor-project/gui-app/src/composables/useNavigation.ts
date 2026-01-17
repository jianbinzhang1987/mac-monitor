import { ref } from 'vue';

const activeView = ref('overview');

export const useNavigation = () => {
    const navigateTo = (viewId: string) => {
        activeView.value = viewId;
    };

    return {
        activeView,
        navigateTo,
    };
};
