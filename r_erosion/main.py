import glob
import matplotlib.pyplot as plt
import numpy as np
import os
import scipy

def gen_clut():
    print('Generating CLUT...')

    clut_res = 32

    dir_list = glob.glob('data/*')
    cmap_list = [os.path.basename(d) for d in dir_list if os.path.isdir(d)]

    
    for cmap in cmap_list:
        print(cmap)

        # See https://worldview.earthdata.nasa.gov to get data
        fname_dem = os.path.join('data', cmap, 'dem.png')
        fname_col = os.path.join('data', cmap, 'aerial.png')

        # CLUT based on elevation and gradient components
        z = load_img(fname_dem)[:, :, 0]
        dz = gradient_norm(z)
        dzx, dzy = np.gradient(z)

        clut3 = generate_clut((z, dzx, dzy),
                                         fname_col=fname_col,
                                         clut_shape=(clut_res,
                                                     clut_res,
                                                     clut_res))

        np.save('clut_3.npy', clut3)
    
    plt.show()



# def clamp(z, vmin=0, vmax=1):
#     return np.minimum(vmax * np.ones(z.shape),
#                       np.maximum(vmin * np.ones(z.shape), z))


def gradient_angle(z):
    dx, dy = np.gradient(z, axis=0), np.gradient(z, axis=1)
    return np.arctan2(dy, dx)


def gradient_norm(z):
    dx, dy = np.gradient(z, axis=0), np.gradient(z, axis=1)
    return np.hypot(dx, dy)


def gaussian_curvature(z, sigma=2):
    z = scipy.ndimage.gaussian_filter(z, sigma=sigma)
    
    zx, zy = np.gradient(z)
    zxx, zxy = np.gradient(zx)
    _, zyy = np.gradient(zy)

    # Gaussian curvature = K1 * K2
    k = (zxx * zyy - (zxy**2)) / (1 + (zx**2) + (zy**2))**2
    # mean curvature = (K1 + K2)/2
    h = (zxx * (1 + zy**2) - 2 * zxy * zx * zy + zyy *
         (1 + zx**2)) / 2 / (1 + zx**2 + zy**2)**1.5

    return k, h


def hillshade(z, azimuth, zenith, talus_ref):
    azimuth_rad = np.pi * azimuth / 180
    zenith_rad = np.pi * zenith / 180

    aspect = gradient_angle(z)
    dn = gradient_norm(z) / talus_ref
    slope = np.arctan(dn)

    sh = np.cos(zenith_rad) * np.cos(slope) \
        + np.sin(zenith_rad) * np.sin(slope) * np.cos(azimuth_rad - aspect)
    
    return (sh - sh.min()) / sh.ptp()




def load_img(fname):
    return (plt.imread(fname) * 255).astype(int)


def generate_clut(features,
                  fname_col,
                  clut_shape):

    nfeatures = len(features)
    
    # reference colors
    img_col = load_img(fname_col)

    # initialize color LUT => clut_shape + 4 color channels
    clut = 255 * np.ones(clut_shape + (4,), dtype=int)

    # prepare grids and data
    lspace = ()
    for i in range(nfeatures):
        lspace += (np.linspace(features[i].min(), features[i].max(), clut_shape[i]),)   
    Xg = np.meshgrid(*lspace, indexing='ij')
    Xitp = ()
    for i in range(nfeatures):
        Xitp += (Xg[i].ravel(),)

    X = ()
    for i in range(nfeatures):
        X += (features[i].ravel(),)
    X = list(zip(*X))
    
    for k in range(3):
        v = img_col[..., k].ravel()
        fitp = scipy.interpolate.NearestNDInterpolator(X, v)
        tk = fitp(*Xitp).reshape(clut_shape)
        tk = scipy.ndimage.gaussian_filter(tk, sigma=1)
        clut[..., k] = tk
        
    return clut


def apply_clut(features, clut):
    img_out = 255 * np.ones((features[0].shape[0], features[0].shape[1], 4), dtype=int)
    nfeatures = len(features)
    clut_shape = clut.shape[:-1]
    
    lspace = ()
    for i in range(nfeatures):
        lspace += (np.linspace(features[i].min(), features[i].max(), clut_shape[i]),)   
    Xg = np.meshgrid(*lspace, indexing='ij')
    X = ()
    for i in range(nfeatures):
        X += (Xg[i].ravel(),)

    Xitp = ()
    for i in range(nfeatures):
        Xitp += (features[i].ravel(),)
    Xitp = list(zip(*Xitp))

    for k in range(3):
        v = clut[..., k].ravel()
        fitp = scipy.interpolate.NearestNDInterpolator(X, v)
        img_out[..., k] = fitp(Xitp).reshape(features[0].shape)

    return img_out

def apply_clut_png():
    print('Applying CLUT...')

    dir_list = glob.glob('data/*')
    cmap_list = [os.path.basename(d) for d in dir_list if os.path.isdir(d)]

    for cmap in cmap_list:
        print(cmap)
        
        # raw DEM
        z = load_img('data/eroded_rgb.png')[:, :, 0]

        # compute features used to colorize
        dz = gradient_norm(z)
        _, dh = gaussian_curvature(z)
        dzx, dzy = np.gradient(z)

        # colorize
        clut3 = np.load('clut_3.npy')
        img3 = apply_clut((z, dzx, dzy), clut3)

        # add hillshading
        # sh = hillshade(z, 180, 45, 10 * z.ptp() / z.shape[0])
        # for k in range(3):
        #     img3[:, :, k] = (sh**0.5 * img3[:, :, k]).astype(int)

        # plt.figure()
        # plt.imshow(img3)
        # plt.axis('off')

        plt.imsave('texture.png', img3.astype(np.uint8))
        
    # plt.show()



