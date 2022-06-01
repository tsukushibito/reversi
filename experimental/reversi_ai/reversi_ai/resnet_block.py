from typing import Dict, Any
import tensorflow as tf
from tensorflow.keras.regularizers import l2
from tensorflow.keras.layers import BatchNormalization, Conv2D


class ResnetBlock(tf.keras.layers.Layer):

    def __init__(self, kernel_size=(3, 3), filter_size=16, stride=1, dif_fsize=False):
        ''' args:
             kernel_size: kernel_size. default is (3,3)
             filter_size: numbers of output filters.
             stride: scalar. If this is not 1, skip connection are replaced to 1x1 convolution.
             dif_fsize: True if the numbers of input filter and output filter are different
        '''
        super().__init__(name='')
        self.kernel_size = kernel_size
        self.filter_size = filter_size
        self.stride = stride
        self.dif_fsize = dif_fsize

        if stride == 1:
            strides = (1, 1)
        else:
            strides = (stride, stride)

        self.bn2a = BatchNormalization()
        self.conv2a = Conv2D(filter_size,
                             kernel_size,
                             strides=strides,
                             padding='same',
                             kernel_initializer='he_normal',
                             kernel_regularizer=l2(0.0005),
                             use_bias=False)

        self.bn2b = BatchNormalization()
        self.conv2b = Conv2D(filter_size,
                             kernel_size,
                             strides=(1, 1),
                             padding='same',
                             kernel_initializer='he_normal',
                             kernel_regularizer=l2(0.0005),
                             use_bias=False)

        self.use_identity_shortcut = (stride == 1) and not dif_fsize
        if not self.use_identity_shortcut:
            self.conv2_sc = tf.keras.layers.Conv2D(
                filter_size,
                (1, 1),
                strides=strides,
                padding='same',
                kernel_initializer='he_normal',
                kernel_regularizer=l2(0.0005),
                use_bias=False)

    def call(self, input_tensor, training=False):
        x = self.bn2a(input_tensor, training=training)
        x1 = tf.nn.relu(x)
        x = self.conv2a(x1)

        x = self.bn2b(x, training=training)
        x = tf.nn.relu(x)
        x = self.conv2b(x)

        if self.use_identity_shortcut:
            skip = input_tensor
        else:
            skip = self.conv2_sc(x1)
        x += skip
        return x

    def get_config(self) -> Dict[str, Any]:
        # config = super().get_config()
        config = {}
        config.update({"kernel_size": self.kernel_size,
                       "filter_size": self.filter_size,
                       "stride": self.stride,
                       "dif_fsize": self.dif_fsize})
        return config
